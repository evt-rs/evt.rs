use std::io;

pub use chrono::{DateTime, Utc};
pub use serde_json::Value as Json;
pub use uuid::Uuid;

use thiserror::Error;

pub use message_store::MessageStore;

pub use crate::clock::Clock;

mod clock;
pub mod db;
pub mod identity;
pub mod message_store;
#[macro_use]
pub mod stream_name;
mod messaging;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error from postgres")]
    PgError(#[from] postgres::Error),
    #[error("postgres io error")]
    PgIoError(#[from] io::Error),
    #[error("serialization error")]
    Serialization(#[from] serde_json::Error),
    #[error("multiple messages found")]
    MultipleMessages(String),
    #[error("{0}")]
    ExpectedVersion(String),
    #[error("missing field in message data")]
    MissingField,
}
