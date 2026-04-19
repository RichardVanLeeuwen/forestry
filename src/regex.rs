use std::sync::LazyLock;

use regex::Regex;

pub static BRANCH_EXISTS_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"fatal: a branch named '.*' already exists").unwrap());

pub fn get_name_from_tree_string(tree_string: &str) -> String {
    // ?<name> give the capture group a name
    let tree_name_regex = Regex::new(r"\[(?<name>.+)\]").unwrap();
    // and we can find the capture by that name
    tree_name_regex.captures(tree_string).unwrap()["name"].to_owned()
}
