use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

pub fn require(requirement: bool, message: &str) -> Result<()> {
    if !requirement {
        return Err(Error::InvalidArgument(message.to_string()));
    }
    return Ok(());
}

pub fn os_error(message: &str) -> Error {
    Error::Os(message.to_string())
}

pub fn not_found(message: &str) -> Error {
    Error::NotFound(message.to_string())
}

pub fn database(message: String) -> Error {
    Error::Database(message)
}

#[derive(Debug, Clone)]
pub enum Error {
    Database(String),
    NotFound(String),
    InvalidArgument(String),
    Os(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Database(s) => write!(f, "{s}"),
            Self::NotFound(s) => write!(f, "{s}"),
            Self::InvalidArgument(s) => write!(f, "{s}"),
            Self::Os(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<tokio::io::Error> for Error {
    fn from(value: tokio::io::Error) -> Self {
        return Self::Os(value.to_string());
    }
}
