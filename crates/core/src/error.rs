use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Page {0} not found")]
    PageNotFound(u64),
    #[error("B-tree is full")]
    BTreeFull,
}
