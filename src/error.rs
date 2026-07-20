#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot find {name} '{value}' in binary")]
    FieldNotFound {
        name: String,
        value: String,
    },

    #[error("Binary already contains {name} '{value}'")]
    FieldAlreadyExists {
        name: String,
        value: String,
    },

    #[error("Error while interacting with filesystem: {0}")]
    IOError(#[from] std::io::Error),
}

pub(super) type Result<T> = std::result::Result<T, Error>;
