use std::process::Command;

use crate::regex::INVALID_REF_REGEX;

pub struct Branch {
    pub name: String,
    pub remote: Option<String>,
    reflog_string: String,
}

impl Branch {
    pub fn new(reflog_string: String) -> Self {
        tracing::info!(reflog_string);
        let (name, remote) = get_branch_info_from_reflog_string(&reflog_string);
        Branch {
            name,
            remote,
            reflog_string,
        }
    }
}

fn get_branch_info_from_reflog_string(reflog: &String) -> (String, Option<String>) {
    let mut iter = reflog.split('/');
    // first part is alway "refs" which is not interesting to us
    iter.next();
    // the next part is either "heads" or "remotes" if it is a local or remote branch
    let is_remote = iter.next() == Some("remotes");
    let remote = if is_remote {
        Some(
            iter.next()
                .expect("there should be another reflog string part when branch is remote")
                .to_string(),
        )
    } else {
        None
    };
    let name = iter
        .next()
        .expect("There should be a branch name")
        .to_string();
    (name, remote)
}

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

pub fn get_branches() -> Vec<Branch> {
    let command = Command::new("git")
        .arg("branch")
        .arg("-la")
        .arg("--format=%(refname)")
        .output()
        .expect("Could not gather git branches");
    String::from_utf8_lossy(&command.stdout)
        .lines()
        .map(|v| {
            tracing::info!(v);
            v.to_string()
        })
        .map(Branch::new)
        .collect()
}

pub fn create_worktree(location: String, branch: String) {
    tracing::info!("creating worktree: {} {}", &location, &branch);
    let mut args = vec!["worktree", "add", "--checkout", &location, &branch];
    let command = Command::new("git")
        .args(&args)
        .output()
        .expect("Could not create worktree");
    let branch_already_exists = String::from_utf8_lossy(&command.stderr)
        .lines()
        .any(|line| INVALID_REF_REGEX.is_match(line));
    if branch_already_exists {
        args[2] = "-b";
        args.swap(3, 4);
        args.iter().for_each(|arg| tracing::info!(arg));
        Command::new("git")
            .args(&args)
            .output()
            .expect("Could not create worktree");
    };
}

pub fn remove_worktree(tree: &str, force: bool) -> bool {
    let tree_name = tree.split_ascii_whitespace().next().unwrap();
    tracing::info!("deleting worktree: {}", tree_name);

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
