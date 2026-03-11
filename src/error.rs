#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error while interacting with filesystem: {0}")]
    IOError(#[from] std::io::Error),
}

pub(super) type Result<T> = std::result::Result<T, Error>;
