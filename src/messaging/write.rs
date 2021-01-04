use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::message_store::controls::message_store;
use crate::message_store::MessageData;
use crate::messaging::Message;
use crate::{message_store, Error, MessageStore};

pub trait Write<T> {
    fn write(
        &mut self,
        batch: T,
        stream_name: &str,
        expected_version: Option<i64>,
    ) -> Result<(), Error>;

    fn write_initial(&mut self, batch: T, stream_name: &str) -> Result<(), Error>;
}

impl<T> Write<&Message<T>> for MessageStore
where
    T: Serialize + DeserializeOwned + Default,
{
    fn write(
        &mut self,
        batch: &Message<T>,
        stream_name: &str,
        expected_version: Option<i64>,
    ) -> Result<(), Error> {
        let data = batch.as_message_data();
        message_store::put::put(&mut self.client, &data, stream_name, expected_version)?;

        Ok(())
    }

    fn write_initial(&mut self, batch: &Message<T>, stream_name: &str) -> Result<(), Error> {
        self.write(batch, stream_name, Some(-1))
    }
}

impl<T> Write<Vec<&Message<T>>> for MessageStore
where
    T: Serialize + DeserializeOwned + Default,
{
    fn write(
        &mut self,
        batch: Vec<&Message<T>>,
        stream_name: &str,
        expected_version: Option<i64>,
    ) -> Result<(), Error> {
        let data: Vec<MessageData> = batch.into_iter().map(|msg| msg.as_message_data()).collect();
        let refs: Vec<&MessageData> = data.iter().collect();

        message_store::put::put_many(&mut self.client, refs, stream_name, expected_version)?;

        Ok(())
    }

    fn write_initial(&mut self, batch: Vec<&Message<T>>, stream_name: &str) -> Result<(), Error> {
        self.write(batch, stream_name, Some(-1))
    }
}
