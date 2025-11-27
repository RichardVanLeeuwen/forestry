use std::{fs, path::Path};

use git2::{Repository, Worktree, WorktreeAddOptions, WorktreePruneOptions};

use crate::app::{App, ListTree};
use crate::error::{Error::UncommittedChanges, Result};

pub fn get_repo() -> Result<Repository> {
    Ok(Repository::open(".")?)
}

pub fn get_worktrees(repo: &Repository) -> Result<Vec<ListTree>> {
    let trees = repo.worktrees().expect("Expected to find worktrees");
    let list_trees = trees
        .iter()
        .flatten()
        .map(|tree_name| {
            let tree = repo
                .find_worktree(tree_name)
                .expect("Expected tree to have a name");
            ListTree {
                name: tree.name().unwrap().to_owned(),
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

pub fn get_branches(repo: &Repository) -> Result<Vec<String>> {
    let branches = repo.branches(None)?;
    Ok(branches
        .filter_map(|b| b.ok())
        .filter_map(|(branch, _)| branch.name().ok().flatten().map(String::from))
        .collect::<Vec<String>>())
}

pub fn create_worktree(branch: String, location: String) -> Result<Worktree> {
    let repo = get_repo().unwrap();
    let add_opts = WorktreeAddOptions::new();
    Ok(repo.worktree(&*branch, Path::new(&location), Some(&add_opts))?)
}

pub fn remove_worktree(repo: &mut App, tree_name: String) -> Result<()> {
    let tree = repo.root.find_worktree(&tree_name)?;
    let path = tree.path().to_path_buf();
    if path.exists() {
        let tree_repo = Repository::open(&path)?;

        let status = tree_repo.statuses(None)?;
        if !status.is_empty() {
            return Err(UncommittedChanges);
        }
    }

    let mut prune_opts = WorktreePruneOptions::new();
    prune_opts.valid(true);
    tree.prune(Some(&mut prune_opts))?;

    if path.exists() {
        fs::remove_dir_all(&path)?;
    }

    Ok(())
}
