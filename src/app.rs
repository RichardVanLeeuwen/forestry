use git2::Repository;
use ratatui::widgets::ListState;

pub enum CurrentScreen {
    Main,
}

fn get_repo() -> Repository {
    match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub root: Repository,
    pub tree_list: TreeList,
}

impl App {
    pub fn new() -> App {
        let root = get_repo();
        App {
            current_screen: CurrentScreen::Main,
            tree_list: TreeList::new(&root),
            root,
        }
    }
}

pub struct ListTree {
    pub branch: String,
    pub location: String,
    pub imaginary: bool,
}

pub struct TreeList {
    pub items: Vec<ListTree>,
    pub state: ListState,
}

impl TreeList {
    pub fn new(repo: &Repository) -> TreeList {
        let trees = repo.worktrees().expect("Expected to find worktrees");
        let list_trees = trees
            .iter()
            .flatten()
            .map(|tree_name| {
                let tree = repo
                    .find_worktree(tree_name)
                    .expect("Expected tree to have a name");
                let tree_repo = Repository::open(tree.path().to_path_buf())
                    .expect("Expected tree to belong to a repo");
                let branch = tree_repo
                    .head()
                    .ok()
                    .as_ref()
                    .and_then(|h| h.target())
                    .map(|oid| oid.to_string())
                    .expect("Expected a branch");
                ListTree {
                    branch,
                    location: tree
                        .path()
                        .to_path_buf()
                        .into_os_string()
                        .into_string()
                        .expect("Expected a location"),
                    imaginary: false,
                }
            })
            .collect();
        let mut state = ListState::default();
        state.select_first();
        TreeList {
            items: list_trees,
            state,
        }
    }
}
