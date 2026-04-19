use std::process::Command;

use crate::regex::{BRANCH_EXISTS_REGEX, get_name_from_tree_string};

pub fn get_root_location() -> String {
    let command = Command::new("git")
        .arg("rev-parse")
        .arg("--path-format=absolute")
        .arg("--git-common-dir")
        .output()
        .expect("Could not find .git dir of repository");
    let mut line = String::from_utf8_lossy(&command.stdout)
        .lines()
        .next()
        .unwrap()
        .to_string();
    line.truncate(line.len() - 5);
    line
}

pub fn get_worktrees() -> Vec<String> {
    let command = Command::new("git")
        .arg("worktree")
        .arg("list")
        .output()
        .expect("Could not get worktree list");
    String::from_utf8_lossy(&command.stdout)
        .lines()
        .map(|v| v.to_string())
        .collect()
}

pub fn get_branches() -> Vec<String> {
    let command = Command::new("git")
        .arg("branch")
        .arg("-l")
        .arg("-a")
        .arg("--format='%(refname:short)'")
        .output()
        .expect("Could not gather git branches");
    String::from_utf8_lossy(&command.stdout)
        .lines()
        .map(|v| v.to_string())
        .collect()
}

pub fn create_worktree(location: String, branch_name: Option<String>) {
    tracing::info!("creating worktree");
    let mut args = vec!["worktree".to_string(), "add".to_string()];
    if let Some(branch) = branch_name {
        args.extend(["-b".to_string(), branch]);
    };
    args.push(location);
    let command = Command::new("git")
        .args(&args)
        .output()
        .expect("Could not create worktree");
    let branch_already_exists = String::from_utf8_lossy(&command.stderr)
        .lines()
        .any(|line| BRANCH_EXISTS_REGEX.is_match(line));
    if branch_already_exists {
        args[2] = "--checkout".to_string();
        args.swap(3, 4);
        Command::new("git")
            .args(&args)
            .output()
            .expect("Could not create worktree");
    };
}

pub fn remove_worktree(tree: &str, force: bool) -> bool {
    tracing::info!("deleting worktree");

    let tree_name = get_name_from_tree_string(tree);

    let mut command = Command::new("git");
    command.arg("worktree").arg("remove");
    if force {
        command.arg("-f");
    }
    let output = command
        .arg(tree_name)
        .output()
        .expect("Failed to remove worktree");
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        tracing::warn!("git worktree remove failed: {}", err);
    }
    output.status.success()
}

pub fn fetch_repo() {
    Command::new("git").arg("fetch");
}
