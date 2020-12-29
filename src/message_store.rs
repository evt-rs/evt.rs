pub mod controls;
mod errors;
pub mod put;

use crate::{DateTime, Json, Utc, Uuid};
use postgres::Client;

pub use errors::MessageStoreError;
pub use put::Put;

#[derive(Default, Clone)]
pub struct MessageData {
    id: Option<Uuid>,
    message_type: String,
    stream_name: Option<String>,
    position: Option<i64>,
    global_position: Option<i64>,
    data: Json,
    metadata: Json,
    time: Option<DateTime<Utc>>,
}

#[derive(Default)]
pub struct Settings {
    batch_size: Option<i32>,
    correlation: Option<String>,
    group_member: Option<i32>,
    group_size: Option<i32>,
    condition: Option<String>,
}

pub struct MessageStore {
    settings: Settings,
    client: Client,
}

impl MessageStore {
    fn new(settings: Settings, client: Client) -> Self {
        MessageStore { settings, client }
    }
}
