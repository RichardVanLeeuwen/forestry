use crossterm::event::{KeyEvent, KeyEventKind};
use git2::Repository;
use ratatui::Terminal;
use ratatui::prelude::Backend;
use ratatui::widgets::ListState;
use tui_input::Input;

use crate::event::{AppEvent, Event, EventHandler};
use crate::git::{get_branches, get_repo, get_worktrees};
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
    pub current_screen: CurrentScreen,
    pub root: Repository,
    pub branch_name: String,
    pub branch_input: Input,
    pub branch_list: BranchList,
    pub worktree_location: Input,
    pub tree_list: TreeList,
    pub creating: Option<CurrentlyCreating>,
    pub ask_force_delete: bool,
}

impl Default for App {
    fn default() -> Self {
        let root = get_repo();
        Self {
            running: true,
            events: EventHandler::new(),
            current_screen: CurrentScreen::Main,
            branch_name: "".to_string(),
            branch_input: Input::default(),
            branch_list: BranchList::new(&root),
            worktree_location: Input::default(),
            tree_list: TreeList::new(&root),
            creating: None,
            root,
            ask_force_delete: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run<B: Backend>(mut self, mut terminal: Terminal<B>) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| ui(frame, &mut self));
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::TreelistUp => self.treelist_up(),
                    AppEvent::TreelistDown => self.treelist_down(),
                    AppEvent::CreateTree => self.create_tree(),
                    AppEvent::DeleteTree => self.delete_tree(),
                },
            }
        }
        Ok(())
    }

    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        mapkey(self, key_event.code);
        Ok(())
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

    fn create_tree(&mut self) {
        self.creating = Some(CurrentlyCreating::Branch);
        self.current_screen = CurrentScreen::Creating;
        self.branch_input = Input::default();
    }

    fn delete_tree(&mut self) {
        self.current_screen = CurrentScreen::Deleting;
    }

    pub fn tick(&self) {}
}

pub struct ListTree {
    pub location: String,
    pub name: String,
}

pub struct TreeList {
    pub items: Vec<ListTree>,
    pub state: ListState,
}

impl TreeList {
    pub fn new(repo: &Repository) -> TreeList {
        let list_trees = get_worktrees(repo);
        let mut state = ListState::default();
        state.select_first();
        TreeList {
            items: list_trees,
            state,
        }
    }
}

pub struct BranchList {
    pub items: Vec<String>,
    pub state: ListState,
}

impl BranchList {
    pub fn new(repo: &Repository) -> BranchList {
        let branches = get_branches(repo);
        let mut state = ListState::default();
        state.select_first();
        BranchList {
            items: branches,
            state,
        }
    }
}
