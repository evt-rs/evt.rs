use postgres::types::ToSql;
use postgres::Client;

use crate::message_store::MessageStoreError;
use crate::message_store::{MessageData, MessageStore, Settings};
use crate::stream_name::is_category;
use crate::{DateTime, Json, Utc, Uuid};
use chrono::NaiveDateTime;

type Params<'a> = &'a [&'a (dyn ToSql + Sync)];
type DataResult = Result<Vec<MessageData>, MessageStoreError>;
type SingleResult = Result<Option<MessageData>, MessageStoreError>;

pub trait Get {
    fn get(&mut self, stream_name: &str, position: Option<i64>) -> DataResult;
    fn get_last(&mut self, stream_name: &str) -> SingleResult;
}

impl Get for MessageStore {
    fn get(&mut self, stream_name: &str, position: Option<i64>) -> DataResult {
        get(&mut self.client, &mut self.settings, stream_name, position)
    }

    fn get_last(&mut self, stream_name: &str) -> SingleResult {
        get_last(&mut self.client, stream_name)
    }
}

pub fn get(
    client: &mut Client,
    settings: &Settings,
    stream_name: &str,
    position: Option<i64>,
) -> DataResult {
    if is_category(stream_name) {
        get_category(client, settings, stream_name, position)
    } else {
        get_stream(client, settings, stream_name, position)
    }
}

pub fn get_last(client: &mut Client, stream_name: &str) -> SingleResult {
    let q: &str = "SELECT * FROM get_last_stream_message($1::varchar)";
    let mut messages = get_messages(client, q, &[&String::from(stream_name)])?;

    match messages.len() {
        1 => Ok(Some(messages.remove(0))),
        0 => Ok(None),
        _ => {
            let stream_name = String::from(stream_name);
            let e = MessageStoreError::MultipleMessages(stream_name);

            Err(e)
        }
    }
}

pub fn get_stream(
    client: &mut Client,
    settings: &Settings,
    stream_name: &str,
    position: Option<i64>,
) -> DataResult {
    let q = "SELECT * FROM \
             get_stream_messages($1::varchar, $2::bigint, $3::bigint, $4::varchar)";

    get_messages(
        client,
        q,
        &[
            &String::from(stream_name),
            &position,
            &settings.batch_size,
            &settings.condition,
        ],
    )
}

fn get_category(
    client: &mut Client,
    settings: &Settings,
    stream_name: &str,
    position: Option<i64>,
) -> DataResult {
    let q = "SELECT * \
             FROM get_category_messages($1::varchar, $2::bigint, $3::bigint, \
                                        $4::varchar, $5::bigint, $6::bigint, \
                                        $7::varchar)";

    get_messages(
        client,
        q,
        &[
            &String::from(stream_name),
            &position,
            &settings.batch_size,
            &settings.correlation,
            &settings.group_member,
            &settings.group_size,
            &settings.condition,
        ],
    )
}

fn get_messages(client: &mut Client, query: &str, params: Params) -> DataResult {
    let results = client
        .query(query, params)?
        .iter()
        .map(|row| MessageData {
            id: uuid_result(row.get(0)),
            stream_name: row.get(1),
            message_type: row.get(2),
            position: row.get(3),
            global_position: row.get(4),
            data: json_result(row.get(5)),
            metadata: json_result(row.get(6)),
            time: time_result(row.get(7)),
        })
        .collect();

    Ok(results)
}

fn uuid_result(result: &str) -> Option<Uuid> {
    match Uuid::parse_str(result) {
        Ok(uuid) => Some(uuid),
        Err(_) => None,
    }
}

fn json_result(result: &str) -> Json {
    serde_json::from_str(result).unwrap()
}

fn time_result(result: NaiveDateTime) -> Option<DateTime<Utc>> {
    Some(DateTime::from_utc(result, Utc))
}

#[cfg(test)]
mod tests {
    use crate::message_store::{controls, Get, MessageData, Put, INITIAL};
    use crate::stream_name;

    #[test]
    fn gets_messages_from_stream() {
        let mut store = controls::message_store();
        let data = controls::new_example();
        let stream_name = stream_name::controls::unique_example();

        let stored = store.put(&data, stream_name.as_str(), INITIAL).unwrap();

        let mut results = store.get(stream_name.as_str(), None).unwrap();

        assert_eq!(1, results.len());

        let retrieved = results.remove(0);

        messages_eq(&stored, &retrieved);
    }

    #[test]
    fn gets_messages_from_category() {
        let mut store = controls::message_store();
        let data = controls::new_example();
        let stream_name = stream_name::controls::unique_category();

        let stored = store.put(&data, stream_name.as_str(), INITIAL).unwrap();

        let mut results = store.get(stream_name.as_str(), None).unwrap();

        assert_eq!(1, results.len());

        let retrieved = results.remove(0);

        messages_eq(&stored, &retrieved);
    }

    #[test]
    fn getting_messages_from_an_empty_stream_results_in_empty_vec() {
        let mut store = controls::message_store();
        let stream_name = stream_name::controls::unique_example();

        let results = store.get(stream_name.as_str(), None).unwrap();

        assert_eq!(0, results.len());
    }

    #[test]
    fn get_the_last_message_from_a_stream() {
        let mut store = controls::message_store();
        let data: Vec<MessageData> = (0..2).map(|_| controls::new_example()).collect();
        let stream_name = stream_name::controls::unique_example();

        let stored = store
            .put_many(data.iter().collect(), stream_name.as_str(), INITIAL)
            .unwrap();

        let retrieved = store.get_last(stream_name.as_str()).unwrap().unwrap();
        let last = stored.last().unwrap();

        messages_eq(&last, &retrieved);
    }

    #[test]
    fn getting_the_last_message_from_an_empty_stream_results_in_none() {
        let mut store = controls::message_store();
        let stream_name = stream_name::controls::unique_example();

        let retrieved = store.get_last(stream_name.as_str()).unwrap();

        assert!(retrieved.is_none());
    }

    fn messages_eq(stored: &MessageData, retrieved: &MessageData) {
        assert_eq!(stored.id, retrieved.id);
        assert_eq!(stored.stream_name, retrieved.stream_name);
        assert_eq!(stored.message_type, retrieved.message_type);
        assert_eq!(stored.position, retrieved.position);
        assert!(retrieved.global_position.is_some());
        assert_eq!(stored.data, retrieved.data);
        assert_eq!(stored.metadata, retrieved.metadata);
        assert!(retrieved.time.is_some());
    }
}
