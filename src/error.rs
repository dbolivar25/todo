use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("todo item not found: {0}")]
    ItemNotFound(String),
    #[error("invalid date format: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("invalid weight format: {0}")]
    WeightParse(String),
    #[error("home directory not found")]
    HomeDirNotFound,
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
