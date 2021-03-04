
use {
    thiserror::Error,
};

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
