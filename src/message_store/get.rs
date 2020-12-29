use crate::message_store::errors::MessageStoreError;
use crate::message_store::*;

use crate::stream_name::is_category;
use postgres::types::ToSql;
use postgres::Row;

type Params<'a> = &'a [&'a (dyn ToSql + Sync)];
type DataResult<'a> = Result<Vec<MessageData>, MessageStoreError<'a>>;
type SingleResult<'a> = Result<Option<MessageData>, MessageStoreError<'a>>;

pub trait Get {
    fn get(mut self, stream_name: &String, position: Option<i64>) -> DataResult;
    fn get_last(mut self, stream_name: &String) -> SingleResult;
}

impl Get for MessageStore {
    fn get(mut self, stream_name: &String, position: Option<i64>) -> DataResult {
        match is_category(stream_name) {
            true => get_category(&mut self, stream_name, position),
            false => get_stream(&mut self, stream_name, position),
        }
    }

    fn get_last(mut self, stream_name: &String) -> SingleResult {
        get_last(&mut self, stream_name)
    }
}

fn get_last(store: &mut MessageStore, stream_name: &String) -> SingleResult {
    let q = "SELECT * FROM get_last_stream_message($1::varchar)";
    let mut messages = get_messages(store, q, &[stream_name])?;

    match messages.len() {
        1 => Ok(Some(messages.remove(0))),
        0 => None,
        _ => MessageStoreError.MultipleMessages(stream_name),
    }
}

fn get_stream(store: &mut MessageStore, stream_name: &str, position: Option<i64>) -> DataResult {
    let q = "SELECT * FROM get_stream_messages($1::varchar, $2::bigint, $3::bigint, $4::varchar)";
    let s = &store.settings;

    get_messages(
        store,
        q,
        &[stream_name, position, &s.batch_size, &s.condition],
    )
}

fn get_category(store: &mut MessageStore, stream_name: &str, position: Option<i64>) -> DataResult {
    let q = "SELECT * FROM get_category_messages($1::varchar, $2::bigint, $3::bigint, $4::varchar, $5::bigint, $6::bigint, $7::varchar)";
    let s = &store.settings;

    get_messages(
        store,
        q,
        &[
            stream_name,
            position,
            &s.batch_size,
            &s.correlation,
            &s.group_member,
            &s.group_size,
            &s.condition,
        ],
    )
}

fn get_messages(store: &mut MessageStore, query: &str, params: Params) -> DataResult {
    perform_query(&mut store.client, &query, params)?
        .iter()
        .map(|row| MessageData {
            id: row.get(1),
            r#type: row.get(3),
            stream_name: row.get(2),
            position: row.get(4),
            global_position: row.get(5),
            data: row.get(6),
            metadata: row.get(7),
            time: row.get(8),
        })
        .collect()
}

fn perform_query(
    client: &mut Client,
    query: &str,
    params: Params,
) -> Result<Vec<Row>, MessageStoreError> {
    match client.query(query, params) {
        Ok(rows) => Ok(rows),
        Err(err) => MessageStoreError.PgError(err),
    }
}
