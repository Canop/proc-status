
use {
    thiserror::Error,
};


/// any error which can be raised in this crate
///
/// If the OS isn't compatible (ie it doesn't have
/// a /proc pseudo file system), you'll get a
/// ProcStatusError::Io.
#[derive(Error, Debug)]
pub enum ProcStatusError {
    #[error("reading /proc file failed")]
    Io(#[from] std::io::Error),
    #[error("failed to parse as integer")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("no colon in line '{0}'")]
    NoColon(String),
    #[error("no entry with key '{0}'")]
    EntryNotFound(String),
    #[error("not a kib entry")]
    NotInKib,
}
