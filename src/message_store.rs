use postgres::Client;

pub struct MessageData {
    id: Option<String>,
    r#type: String,
    stream_name: String,
    position: Option<i64>,
    global_position: Option<i64>,
    data: String,
    metadata: String,
    time: std::time::Instant,
}

pub struct Settings {
    batch_size: Option<i32>,
    correlation: Option<String>,
    group_member: Option<i32>,
    group_size: Option<i32>,
    condition: Option<String>,
}

pub struct MessageStore {
    settings: Option<Settings>,
    client: Option<Client>,
}
