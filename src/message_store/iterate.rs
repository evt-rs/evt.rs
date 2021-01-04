use crate::message_store::{MessageData, Settings};
use crate::{message_store, MessageStore};
use postgres::Client;

trait Iterate {
    fn iterate(&mut self, stream_name: &str, position: Option<i64>, settings: Option<Settings>);
}

impl Iterate for MessageStore {
    fn iterate(&mut self, stream_name: &str, position: Option<i64>, settings: Option<Settings>) {}
}

pub struct MessageDataIter {
    settings: Settings,
    client: Client,
    stream: String,
    data: Vec<MessageData>,
    position: Option<i64>,
}

impl Iterator for MessageDataIter {
    type Item = MessageData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            match message_store::get::get(
                &mut self.client,
                &mut self.settings,
                self.stream.as_str(),
                self.position,
            ) {
                Some(results) => self.data = results,
                None => return None,
            }
        }

        let last = self.data.pop().unwrap();

        Some(last)
    }
}
