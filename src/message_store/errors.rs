use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageStoreError {
    #[error("error from postgres")]
    PgError(#[from] postgres::Error),
    #[error("multiple messages found")]
    MultipleMessages(String),
    #[error("{0}")]
    ExpectedVersion(String),
}
