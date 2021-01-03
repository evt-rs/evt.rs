use crate::message_store::MessageData;
use crate::messaging::Metadata;
use crate::Uuid;
use core::{fmt, ops};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Message<T: Serialize + DeserializeOwned + Default>(
    pub T,
    pub Option<Uuid>,
    pub Metadata,
);

impl<T> Message<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    pub fn follow<M>(message: &Message<M>) -> Self
    where
        M: Serialize + DeserializeOwned + Default,
    {
        let metadata = Metadata::follow(message.metadata());
        let from: &M = message;
        let from_value = serde_json::to_value(from).unwrap();
        let data: T = serde_json::from_value(from_value).unwrap();

        Message(data, None, metadata)
    }

    pub fn follows<M>(&self, other: &Message<M>) -> bool
    where
        M: Serialize + DeserializeOwned + Default,
    {
        self.metadata().follows(other.metadata())
    }

    pub fn correlated(&self, stream: &str) -> bool {
        self.metadata().correlated(stream)
    }

    pub fn correlate(&mut self, stream: &str) {
        self.2.correlate(stream);
    }

    pub fn message_id(&self) -> &Option<Uuid> {
        &self.1
    }

    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn into_message_data(self) -> MessageData {
        let data = self.0;
        let id = self.1;
        let metadata = self.2;

        MessageData {
            id,
            message_type: String::from(type_name::<T>()),
            stream_name: metadata.stream_name.clone(),
            position: metadata.position,
            global_position: metadata.global_position,
            data: serde_json::to_value(&data).unwrap(),
            metadata: serde_json::to_value(&metadata).unwrap(),
            time: metadata.time,
        }
    }

    pub fn as_message_data(&self) -> MessageData {
        let data = &self.0;
        let id = self.1;
        let metadata = &self.2;

        MessageData {
            id,
            message_type: String::from(type_name::<T>()),
            stream_name: metadata.stream_name.clone(),
            position: metadata.position,
            global_position: metadata.global_position,
            data: serde_json::to_value(&data).unwrap(),
            metadata: serde_json::to_value(&metadata).unwrap(),
            time: metadata.time,
        }
    }

    pub fn metadata(&self) -> &Metadata {
        &self.2
    }
}

impl<T> From<MessageData> for Message<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    fn from(value: MessageData) -> Self {
        let id = value.id;
        let mut metadata = Metadata::from(&value);
        metadata.message_type = Some(String::from(type_name::<T>()));
        let val: T = serde_json::from_value(value.data).unwrap();

        Message(val, id, metadata)
    }
}

impl<T> ops::Deref for Message<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for Message<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> fmt::Debug for Message<T>
where
    T: Serialize + DeserializeOwned + Default + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> fmt::Display for Message<T>
where
    T: Serialize + DeserializeOwned + Default + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

fn type_name<T>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.split("::").into_iter().last().unwrap_or(name)
}

#[cfg(test)]
mod tests {
    use crate::message_store;
    use crate::messaging::controls::message as controls;
    use crate::messaging::controls::message::Event;
    use crate::messaging::Message;
    use crate::stream_name;
    use std::convert::TryFrom;

    #[test]
    fn following_copies_attributes() {
        let cmd = controls::command();
        let field1 = controls::field1();
        let field2 = controls::field2();

        let evt: Message<controls::Event> = Message::follow(&cmd);

        assert_eq!(field1, evt.field1);
        assert_eq!(field2, evt.field2);
        assert_eq!(String::default(), evt.field3);
    }

    #[test]
    fn follows() {
        let cmd = controls::command();

        let evt: Message<controls::Event> = Message::follow(&cmd);

        assert!(evt.follows(&cmd));
    }

    #[test]
    fn correlates() {
        let mut cmd = controls::command();
        let stream = stream_name::controls::example();

        cmd.correlate(&stream);

        assert!(cmd.correlated(&stream));
    }

    #[test]
    fn converts_from_message_data() {
        let message_data = message_store::controls::example();

        let id = message_data.id;
        let message_type = message_data.message_type.clone();
        let position = message_data.position;
        let global_position = message_data.global_position;
        let field1 = controls::field1();
        let field2 = controls::field2();
        let field3 = controls::field3();

        let message = Message::<Event>::try_from(message_data).unwrap();
        let metadata = message.metadata();

        assert_eq!(&id, message.message_id());

        assert_eq!(message_type, metadata.message_type.clone().unwrap());
        assert_eq!(position, metadata.position);
        assert_eq!(global_position, metadata.global_position);

        assert_eq!(field1, message.field1);
        assert_eq!(field2, message.field2);
        assert_eq!(field3, message.field3);
    }
}
