use std::sync::LazyLock;

use regex::Regex;

pub static INVALID_REF_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^fatal: invalid reference:").unwrap());
