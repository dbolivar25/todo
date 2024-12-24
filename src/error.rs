use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Todo item not found: {0}")]
    ItemNotFound(String),
    #[error("Invalid date format: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("Invalid weight format: {0}")]
    WeightParse(String),
    #[error("Home directory not found")]
    HomeDirNotFound,
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
