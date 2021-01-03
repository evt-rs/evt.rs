use crate::message_store::core::{MessageData, MessageStore, Settings};
use crate::{clock, db, identity, messaging, stream_name, Uuid};

pub fn settings() -> Settings {
    Settings {
        ..Default::default()
    }
}

pub fn message_store() -> MessageStore {
    MessageStore {
        client: db::build(),
        settings: settings(),
    }
}

pub fn example() -> MessageData {
    MessageData {
        id: id(),
        message_type: message_type(),
        position: position(),
        global_position: global_position(),
        data: messaging::controls::message::data(),
        metadata: messaging::controls::message::metadata(),
        stream_name: Some(stream_name::controls::example()),
        time: Some(clock::controls::time()),
    }
}

pub fn new_example() -> MessageData {
    MessageData {
        message_type: message_type(),
        data: messaging::controls::message::data(),
        metadata: messaging::controls::message::metadata(),
        ..Default::default()
    }
}

pub fn message_type() -> String {
    String::from("Event")
}

pub fn position() -> Option<i64> {
    Some(10)
}

pub fn global_position() -> Option<i64> {
    Some(20)
}

pub fn id() -> Option<Uuid> {
    Some(identity::uuid())
}
