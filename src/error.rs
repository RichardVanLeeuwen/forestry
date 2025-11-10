use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Git2Error(git2::Error),
    IoError(std::io::Error),
    UncommittedChanges,
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Error {
        Error::Git2Error(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IoError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Git2Error(e) => {
                println!("{:?}", e);
                write!(f, "{}", e)
            }
            Error::IoError(e) => write!(f, "{}", e),
            Error::UncommittedChanges => write!(f, "Uncommitted changes found in worktree."),
        }
    }
}
