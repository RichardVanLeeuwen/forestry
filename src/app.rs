use git2::Repository;
use ratatui::widgets::ListState;
use tui_input::Input;

use crate::error::Result;
use crate::git::{get_branches, get_repo, get_worktrees};

pub enum CurrentScreen {
    Main,
    Creating,
}

pub enum CurrentlyCreating {
    Location,
    Branch,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub root: Repository,
    pub branch_name: String,
    pub branch_input: Input,
    pub branch_list: BranchList,
    pub worktree_location: Input,
    pub tree_list: TreeList,
    pub creating: Option<CurrentlyCreating>,
    pub logging: Vec<String>,
}

impl App {
    pub fn new() -> Result<App> {
        let root = get_repo()?;
        Ok(App {
            current_screen: CurrentScreen::Main,
            branch_name: "".to_string(),
            branch_input: Input::default(),
            branch_list: BranchList::new(&root)?,
            worktree_location: Input::default(),
            tree_list: TreeList::new(&root)?,
            creating: None,
            root,
            logging: Vec::new(),
        })
    }
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
    pub fn new(repo: &Repository) -> Result<TreeList> {
        let list_trees = get_worktrees(repo)?;
        let mut state = ListState::default();
        state.select_first();
        Ok(TreeList {
            items: list_trees,
            state,
        })
    }
}

pub struct BranchList {
    pub items: Vec<String>,
    pub state: ListState,
}

impl BranchList {
    pub fn new(repo: &Repository) -> Result<BranchList> {
        let branches = get_branches(repo)?;
        let mut state = ListState::default();
        state.select_first();
        Ok(BranchList {
            items: branches,
            state,
        })
    }
}
