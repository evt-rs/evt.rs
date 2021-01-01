use crate::message_store::MessageData;
use crate::Uuid;
use core::{fmt, ops};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::TryFrom;

pub struct Message<T: Serialize + DeserializeOwned + Default>(pub T, Option<Uuid>);

pub fn follow<T, M>(message: &Message<M>) -> Result<Message<T>, serde_json::Error>
where
    T: Serialize + DeserializeOwned + Default,
    M: Serialize + DeserializeOwned + Default,
{
    let from: &M = message;
    let from_value = serde_json::to_value(from)?;
    let data: T = serde_json::from_value(from_value)?;

    Ok(Message(data, None))
}

impl<T> Message<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    pub fn message_id(&self) -> &Option<Uuid> {
        &self.1
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> TryFrom<MessageData> for Message<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    type Error = serde_json::Error;

    fn try_from(value: MessageData) -> Result<Self, Self::Error> {
        let id = value.id;
        let val: T = serde_json::from_value(value.data)?;

        Ok(Message(val, id))
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
    use super::Message;
    use serde::{Deserialize, Serialize};

    #[derive(Default, Serialize, Deserialize)]
    struct MyCommand {
        my_val: bool,
        my_different_val: bool,
    }

    impl MyCommand {
        fn val(&self) -> bool {
            self.my_val
        }
    }

    #[derive(Default, Serialize, Deserialize)]
    #[serde(default)]
    struct MyEvent {
        my_val: bool,
        my_other_val: bool,
    }

    #[test]
    fn fields_can_be_dereferenced() {
        let val = MyCommand {
            my_val: true,
            ..Default::default()
        };
        let msg = Message(val, None);

        assert!(msg.my_val);
    }

    #[test]
    fn methods_can_be_dereferenced() {
        let val = MyCommand {
            my_val: true,
            ..Default::default()
        };
        let msg = Message(val, None);

        assert!(msg.val());
    }

    #[test]
    fn can_coerce() {
        let val = MyCommand {
            my_val: true,
            ..Default::default()
        };
        let msg = Message(val, None);
        let result: &MyCommand = &msg;

        assert!(result.my_val);
    }

    #[test]
    fn converts_to_inner_value() {
        let val = MyCommand {
            my_val: true,
            ..Default::default()
        };
        let msg = Message(val, None);
        let result = msg.into_inner();

        assert!(result.my_val);
    }

    mod following {
        use super::super::{follow, Message};
        use super::*;

        #[test]
        fn copies_attributes() {
            let cmd_data = MyCommand {
                my_val: true,
                ..Default::default()
            };

            let cmd = Message(cmd_data, None);

            let evt: Message<MyEvent> = follow(&cmd).unwrap();

            assert_eq!(true, evt.my_val);
            assert_eq!(false, evt.my_other_val);
        }
    }
}
