use crate::message_store::MessageData;
use crate::messaging::Metadata;
use crate::Uuid;
use core::{fmt, ops};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::TryFrom;

pub struct Message<T: Serialize + DeserializeOwned + Default>(
    pub T,
    pub Option<Uuid>,
    pub Metadata,
);

impl<T> Message<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    pub fn follow<M>(message: &Message<M>) -> Result<Self, serde_json::Error>
    where
        M: Serialize + DeserializeOwned + Default,
    {
        let metadata = Metadata::follow(message.metadata());
        let from: &M = message;
        let from_value = serde_json::to_value(from)?;
        let data: T = serde_json::from_value(from_value)?;

        Ok(Message(data, None, metadata))
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

    pub fn metadata(&self) -> &Metadata {
        &self.2
    }
}

impl<T> TryFrom<MessageData> for Message<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    type Error = serde_json::Error;

    fn try_from(value: MessageData) -> Result<Self, Self::Error> {
        let id = value.id;
        let metadata = Metadata::from(&value);
        let val: T = serde_json::from_value(value.data)?;

        Ok(Message(val, id, metadata))
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

#[cfg(test)]
mod tests {
    use crate::messaging::controls::message as controls;
    use crate::messaging::Message;
    use crate::stream_name;

    #[test]
    fn following_copies_attributes() {
        let cmd = controls::command();
        let field1 = controls::field1();
        let field2 = controls::field2();

        let evt: Message<controls::Event> = Message::follow(&cmd).unwrap();

        assert_eq!(field1, evt.field1);
        assert_eq!(field2, evt.field2);
        assert_eq!(String::default(), evt.field3);
    }

    #[test]
    fn follows() {
        let cmd = controls::command();

        let evt: Message<controls::Event> = Message::follow(&cmd).unwrap();

        assert!(evt.follows(&cmd));
    }

    #[test]
    fn correlates() {
        let mut cmd = controls::command();
        let stream = stream_name::controls::example();

        cmd.correlate(&stream);

        assert!(cmd.correlated(&stream));
    }
}
