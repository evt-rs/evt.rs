use crate::messaging::Metadata;
use crate::{clock, message_store, DateTime, Utc};

pub fn example() -> Metadata {
    Metadata {
        stream_name: Some(stream()),
        position: Some(position()),
        global_position: Some(global_position()),
        message_type: Some(message_type()),
        causation_message_stream_name: Some(causation_message_stream_name()),
        causation_message_position: Some(causation_message_position()),
        causation_message_global_position: Some(causation_message_global_position()),
        correlation_stream_name: Some(correlation_stream_name()),
        reply_stream_name: Some(reply_stream_name()),
        time: Some(time()),
        schema_version: Some(schema_version()),
    }
}

pub fn empty() -> Metadata {
    Metadata {
        ..Default::default()
    }
}

pub fn stream() -> String {
    String::from("stream")
}

pub fn message_type() -> String {
    message_store::controls::message_type()
}

pub fn position() -> i64 {
    10
}

pub fn global_position() -> i64 {
    20
}

pub fn causation_message_stream_name() -> String {
    String::from("causationStream")
}

pub fn causation_message_position() -> i64 {
    15
}

pub fn causation_message_global_position() -> i64 {
    25
}

pub fn correlation_stream_name() -> String {
    String::from("correlationStream")
}

pub fn reply_stream_name() -> String {
    String::from("replyStream")
}

pub fn time() -> DateTime<Utc> {
    clock::controls::time()
}

pub fn schema_version() -> String {
    String::from("1")
}
