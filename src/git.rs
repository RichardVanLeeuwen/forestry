use std::path::Path;

use git2::{Repository, Worktree, WorktreeAddOptions};

use crate::app::ListTree;

pub fn get_repo() -> Result<Repository, git2::Error> {
    Repository::open(".")
}

pub fn get_worktrees(repo: &Repository) -> Result<Vec<ListTree>, git2::Error> {
    let trees = repo.worktrees().expect("Expected to find worktrees");
    let list_trees = trees
        .iter()
        .flatten()
        .map(|tree_name| {
            let tree = repo
                .find_worktree(tree_name)
                .expect("Expected tree to have a name");
            ListTree {
                location: tree
                    .path()
                    .to_path_buf()
                    .into_os_string()
                    .into_string()
                    .expect("Expected a location"),
            }
        })
        .collect();
    Ok(list_trees)
}

pub fn create_worktree(branch: String, location: String) -> Result<Worktree, git2::Error> {
    let repo = get_repo().unwrap();
    let add_opts = WorktreeAddOptions::new();
    repo.worktree(&*branch, Path::new(&location), Some(&add_opts))
}
