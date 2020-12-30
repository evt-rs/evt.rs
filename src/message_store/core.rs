use crate::Json;
use crate::Uuid;
use crate::{DateTime, Utc};
use postgres::Client;

pub const INITIAL: Option<i64> = Some(-1);

#[derive(Default, Clone)]
pub struct MessageData {
    pub id: Option<Uuid>,
    pub message_type: String,
    pub stream_name: Option<String>,
    pub position: Option<i64>,
    pub global_position: Option<i64>,
    pub data: Json,
    pub metadata: Json,
    pub time: Option<DateTime<Utc>>,
}

#[derive(Default)]
pub struct Settings {
    pub batch_size: Option<i64>,
    pub correlation: Option<String>,
    pub group_member: Option<i64>,
    pub group_size: Option<i64>,
    pub condition: Option<String>,
}

pub struct MessageStore {
    pub settings: Settings,
    pub client: Client,
}
