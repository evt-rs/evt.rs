use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageStoreError {
    #[error("error from postgres")]
    PgError(#[from] postgres::Error),
    #[error("postgres io error")]
    PgIoError(#[from] io::Error),
    #[error("multiple messages found")]
    MultipleMessages(String),
    #[error("{0}")]
    ExpectedVersion(String),
    #[error("missing field in message data")]
    MissingField,
}
