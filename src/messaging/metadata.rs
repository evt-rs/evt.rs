use crate::message_store::MessageData;
use crate::stream_name;
use crate::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Metadata {
    #[serde(skip)]
    pub stream_name: Option<String>,
    #[serde(skip)]
    pub message_type: Option<String>,
    #[serde(skip)]
    pub position: Option<i64>,
    #[serde(skip)]
    pub global_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causation_message_stream_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causation_message_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causation_message_global_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_stream_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_stream_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

impl Metadata {
    pub fn follow(other: &Metadata) -> Metadata {
        Metadata {
            causation_message_stream_name: other.causation_message_stream_name.clone(),
            causation_message_position: other.causation_message_position,
            causation_message_global_position: other.causation_message_global_position,
            reply_stream_name: other.reply_stream_name.clone(),
            ..Default::default()
        }
    }

    pub fn follows(&self, other: &Metadata) -> bool {
        self.causation_message_stream_name == other.causation_message_stream_name
            && self.causation_message_position == other.causation_message_position
            && self.causation_message_global_position == other.causation_message_global_position
            && self.reply_stream_name == other.reply_stream_name
    }

    pub fn correlate(&mut self, stream_name: &str) {
        self.correlation_stream_name = Some(String::from(stream_name))
    }

    pub fn correlated(&self, stream_name: &str) -> bool {
        if self.correlation_stream_name.is_none() {
            return false;
        }

        let correlation_stream_name = self.correlation_stream_name.as_ref().unwrap();

        stream_name::get_category(correlation_stream_name) == stream_name::get_category(stream_name)
    }
}

impl From<&MessageData> for Metadata {
    fn from(data: &MessageData) -> Metadata {
        Metadata {
            stream_name: data.stream_name.clone(),
            position: data.position.clone(),
            global_position: data.global_position.clone(),
            time: data.time.clone(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::controls;
    use crate::messaging::Metadata;
    use crate::{identity, stream_name};

    #[test]
    fn correlate_sets_correlation_stream_name() {
        let mut metadata = controls::metadata::empty();
        let stream = controls::metadata::stream();

        metadata.correlate(&stream);

        assert_eq!(stream, metadata.correlation_stream_name.unwrap());
    }

    #[test]
    fn correlated_when_categories_match() {
        let category = controls::metadata::stream();

        let correlation = stream_name!(&category, id = identity::uuid());
        let test = stream_name!(&category, id = identity::uuid());

        let mut metadata = controls::metadata::empty();
        metadata.correlate(&correlation);

        assert!(metadata.correlated(&test));
    }

    #[test]
    fn not_correlated_when_categories_dont_match() {
        let category1 = stream_name::controls::unique_category();
        let category2 = stream_name::controls::unique_category();

        let correlation = stream_name!(&category1, id = identity::uuid());
        let test = stream_name!(&category2, id = identity::uuid());

        let mut metadata = controls::metadata::empty();
        metadata.correlate(&correlation);

        assert!(!metadata.correlated(&test));
    }

    #[test]
    fn following_copies_fields() {
        let other = controls::metadata::example();

        let metadata = Metadata::follow(&other);

        let causation_message_stream_name = other.causation_message_stream_name;
        let causation_message_position = other.causation_message_position;
        let causation_message_global_position = other.causation_message_global_position;
        let reply_stream_name = other.reply_stream_name;

        assert_eq!(
            metadata.causation_message_stream_name,
            causation_message_stream_name
        );
        assert_eq!(
            metadata.causation_message_position,
            causation_message_position
        );
        assert_eq!(
            metadata.causation_message_global_position,
            causation_message_global_position
        );
        assert_eq!(metadata.reply_stream_name, reply_stream_name);
    }
}
