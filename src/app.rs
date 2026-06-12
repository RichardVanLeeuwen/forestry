use ratatui::Terminal;
use ratatui::crossterm::event::{Event as CrossTermEvent, KeyEvent, KeyEventKind};
use ratatui::prelude::Backend;
use ratatui::widgets::ListState;
use throbber_widgets_tui::ThrobberState;
use tokio::sync::mpsc;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler as CrosstermEventHandler;

use crate::event::{AppEvent, Event, EventHandler};
use crate::git::{
    Branch, create_worktree, fetch_repo, get_branches, get_root_location, get_worktrees,
    remove_worktree,
};
use crate::keymapping::mapkey;
use crate::ui::ui;

pub enum CurrentScreen {
    Main,
    Creating,
    Deleting,
}

pub enum CurrentlyCreating {
    Location,
    Branch,
}

pub struct App {
    pub running: bool,
    pub events: EventHandler,
    pub root_location: String,
    pub current_screen: CurrentScreen,
    pub branch_name: String,
    pub branch_input: Input,
    pub branch_list: BranchList,
    pub worktree_location: Input,
    pub tree_list: TreeList,
    pub creating: Option<CurrentlyCreating>,
    pub ask_force_delete: bool,
    pub del_op_sender: mpsc::UnboundedSender<tokio::sync::oneshot::Receiver<bool>>,
    pub deletion_in_progress: bool,
    pub throbber_state: throbber_widgets_tui::ThrobberState,
}

impl Default for App {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            running: true,
            events: EventHandler::new(receiver),
            root_location: get_root_location(),
            current_screen: CurrentScreen::Main,
            branch_name: String::new(),
            branch_input: Input::default(),
            branch_list: BranchList::new(),
            worktree_location: Input::default(),
            tree_list: TreeList::new(),
            creating: None,
            ask_force_delete: false,
            del_op_sender: sender,
            deletion_in_progress: false,
            throbber_state: ThrobberState::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run<B: Backend>(mut self, mut terminal: Terminal<B>) -> color_eyre::Result<()> {
        while self.running {
            let _ = terminal.draw(|frame| ui(frame, &mut self));
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    ratatui::crossterm::event::Event::Key(key_event)
                        if key_event.kind == KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event, event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::TreelistUp => self.treelist_up(),
                    AppEvent::TreelistDown => self.treelist_down(),
                    AppEvent::EnterCreating => self.enter_creating(),
                    AppEvent::ExitCreating => self.exit_creating(),
                    AppEvent::SelectBranchname => self.select_branchname(),
                    AppEvent::BranchListUp => self.branchlist_up(),
                    AppEvent::BranchListDown => self.branchlist_down(),
                    AppEvent::TypeBranchName(key_event) => self.type_branch_name(key_event),
                    AppEvent::CreateTree => self.create_tree(),
                    AppEvent::EnterDeleting => self.enter_deleting(),
                    AppEvent::ForceDeleteTree => self.delete_tree(true),
                    AppEvent::CancelDeleting => self.cancel_deleting(),
                    AppEvent::DeleteTree => self.delete_tree(false),
                },
                Event::WorktreeDeleted(success) => self.after_worktree_deleted(success),
            }
        }
        Ok(())
    }

    pub fn handle_key_events(
        &mut self,
        key_event: KeyEvent,
        event: CrossTermEvent,
    ) -> color_eyre::Result<()> {
        mapkey(self, key_event.code, event);
        Ok(())
    }

    pub fn tick(&mut self) {
        self.throbber_state.calc_next();
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn treelist_up(&mut self) {
        self.tree_list.state.select_previous();
    }

    fn treelist_down(&mut self) {
        self.tree_list.state.select_next();
    }

    fn enter_creating(&mut self) {
        self.creating = Some(CurrentlyCreating::Branch);
        self.current_screen = CurrentScreen::Creating;
        self.branch_input = Input::default();
    }

    fn exit_creating(&mut self) {
        if let Some(creating) = &self.creating {
            match creating {
                CurrentlyCreating::Branch => {
                    self.creating = None;
                    self.current_screen = CurrentScreen::Main;
                }
                CurrentlyCreating::Location => {
                    self.creating = Some(CurrentlyCreating::Branch);
                }
            }
        }
    }

    fn select_branchname(&mut self) {
        let selected_branch_index = self.branch_list.state.selected().unwrap();
        let branch_name = if selected_branch_index == 0 {
            self.branch_input.value()
        } else {
            &self
                .branch_list
                .items
                .iter()
                .map(|b| b.name.clone())
                .filter(|b| b.contains(self.branch_input.value()))
                .take(selected_branch_index)
                .last()
                .unwrap()
                .split('/')
                .last()
                .unwrap()
                .to_string()
        };
        self.worktree_location = Input::default().with_value(format!("../{branch_name}"));
        self.branch_name = branch_name.to_string();
        self.creating = Some(CurrentlyCreating::Location);
    }

    fn branchlist_up(&mut self) {
        self.branch_list.state.select_previous();
    }

    fn branchlist_down(&mut self) {
        self.branch_list.state.select_next();
    }

    fn type_branch_name(&mut self, key_event: CrossTermEvent) {
        if let Some(creating) = &self.creating {
            match creating {
                CurrentlyCreating::Branch => {
                    self.branch_input.handle_event(&key_event);
                }
                CurrentlyCreating::Location => {
                    self.worktree_location.handle_event(&key_event);
                }
            };
        }
    }

    fn create_tree(&mut self) {
        self.branch_input.value_and_reset();
        let worktree_location = self.worktree_location.value_and_reset();
        create_worktree(worktree_location, std::mem::take(&mut self.branch_name));
        self.tree_list = TreeList::new();
        self.creating = None;
        self.current_screen = CurrentScreen::Main;
    }

    fn enter_deleting(&mut self) {
        self.current_screen = CurrentScreen::Deleting;
    }

    fn refresh_branchlist(&mut self) {
        self.tree_list = TreeList::new();
    }

    fn cancel_deleting(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.ask_force_delete = false;
    }

    fn delete_tree(&mut self, force: bool) {
        self.deletion_in_progress = true;
        let tree_name = self
            .tree_list
            .items
            .get(self.tree_list.state.selected().unwrap())
            .unwrap()
            .clone();
        // create a oneshot channel to be able to signal when deleting is done
        let (tx, rx) = tokio::sync::oneshot::channel();
        // send the receiver to the EventTask
        let _ = self.del_op_sender.send(rx);
        tokio::task::spawn_blocking(move || {
            // delete the worktree
            let result = remove_worktree(&tree_name, force);
            // send message that deleting is done
            let _ = tx.send(result);
        });
    }

    fn after_worktree_deleted(&mut self, success: bool) {
        if success {
            fetch_repo();
            self.refresh_branchlist();
            self.current_screen = CurrentScreen::Main;
            self.ask_force_delete = false;
        } else {
            self.ask_force_delete = true;
        }
        self.deletion_in_progress = false;
    }
}

pub struct TreeList {
    pub items: Vec<String>,
    pub state: ListState,
}

impl TreeList {
    pub fn new() -> TreeList {
        let list_trees = get_worktrees();
        let mut state = ListState::default();
        state.select_first();
        TreeList {
            items: list_trees,
            state,
        }
    }
}

pub struct BranchList {
    pub items: Vec<Branch>,
    pub state: ListState,
}

impl BranchList {
    pub fn new() -> BranchList {
        let branches = get_branches();
        let mut state = ListState::default();
        state.select_first();
        BranchList {
            items: branches,
            state,
        }
    }
}
