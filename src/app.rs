use git2::Repository;
use ratatui::widgets::ListState;

use crate::error::Result;
use crate::git::{get_repo, get_worktrees};

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
    pub tree_list: TreeList,
    pub branch_name: String,
    pub worktree_location: String,
    pub creating: Option<CurrentlyCreating>,
    pub logging: Vec<String>,
}

impl App {
    pub fn new() -> Result<App> {
        let root = get_repo()?;
        Ok(App {
            current_screen: CurrentScreen::Main,
            tree_list: TreeList::new(&root)?,
            branch_name: String::new(),
            worktree_location: String::new(),
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
