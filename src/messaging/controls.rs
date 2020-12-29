use crate::Json;

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
