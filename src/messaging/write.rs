use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::message_store::{MessageData, Put};
use crate::messaging::Message;
use crate::{Error, MessageStore};

pub trait Write<T, D, R>: Put<D, R> {
    fn write(
        &mut self,
        batch: T,
        stream_name: &str,
        expected_version: Option<i64>,
    ) -> Result<(), Error>;

    fn write_initial(&mut self, batch: T, stream_name: &str) -> Result<(), Error>;
}

impl<T> Write<&Message<T>, &MessageData, MessageData> for MessageStore
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
        self.put(&data, stream_name, expected_version)?;

        Ok(())
    }

    fn write_initial(&mut self, batch: &Message<T>, stream_name: &str) -> Result<(), Error> {
        self.write(batch, stream_name, Some(-1))
    }
}

impl<T> Write<Vec<&Message<T>>, Vec<&MessageData>, Vec<MessageData>> for MessageStore
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

        self.put(refs, stream_name, expected_version)?;

        Ok(())
    }

    fn write_initial(&mut self, batch: Vec<&Message<T>>, stream_name: &str) -> Result<(), Error> {
        self.write(batch, stream_name, Some(-1))
    }
}
