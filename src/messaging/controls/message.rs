use crate::message_store;
use crate::messaging::{controls, Message};
use crate::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod new {
    use super::{Command, Event};
    use crate::messaging::{controls, Message};

    pub fn example() -> Message<Event> {
        event()
    }

    pub fn command() -> Message<Command> {
        let metadata = controls::metadata::empty();
        let cmd = Command {
            ..Default::default()
        };

        Message(cmd, None, metadata)
    }

    pub fn event() -> Message<Event> {
        let metadata = controls::metadata::empty();
        let evt = Event {
            ..Default::default()
        };

        Message(evt, None, metadata)
    }
}

pub fn example() -> Message<Event> {
    event()
}

pub fn command() -> Message<Command> {
    let metadata = controls::metadata::empty();

    let cmd = Command {
        field1: field1(),
        field2: field2(),
    };

    Message(cmd, id(), metadata)
}

pub fn event() -> Message<Event> {
    let metadata = controls::metadata::empty();

    let evt = Event {
        ..Default::default()
    };

    Message(evt, id(), metadata)
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Command {
    pub field1: String,
    pub field2: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Event {
    pub field1: String,
    pub field2: String,
    pub field3: String,
}

pub fn id() -> Option<Uuid> {
    message_store::controls::id()
}

pub fn field1() -> String {
    String::from("Field1")
}

pub fn field2() -> String {
    String::from("Field2")
}

pub fn field3() -> String {
    String::from("Field3")
}

pub fn data() -> Json {
    Json::from("{\"Field1\": \"Field1\", \"Field2\": \"Field2\", \"Field3\": \"Field3\"}")
}

pub fn metadata() -> Json {
    Json::from("{\"time\": \"2020-10-05T01:02:03.000000004Z\", \"schema_version\": \"1\", \"reply_stream_name\": \"replyStream\", \"correlation_stream_name\": \"correlationStream\", \"causation_message_stream\": \"causationStream\", \"causation_message_position\": 5, \"causation_message_global_position\": 15}")
}
