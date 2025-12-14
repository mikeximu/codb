use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("key not found")]
    NotFound,

    #[error("database is closed")]
    Closed,

    #[error("operation not supported")]
    NotSupported,
}