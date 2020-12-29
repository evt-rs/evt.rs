use crate::identity;
use crate::message_store::errors::MessageStoreError;
use crate::message_store::{MessageData, MessageStore};
use postgres::types::ToSql;
use postgres::GenericClient;

pub type Params<'a> = &'a [&'a (dyn ToSql + Sync)];

pub trait Put {
    fn put(
        &mut self,
        data: &MessageData,
        stream_name: &str,
        expected_version: Option<i64>,
    ) -> Result<MessageData, MessageStoreError>;

    fn put_many(
        &mut self,
        data: Vec<&MessageData>,
        stream_name: &str,
        expected_version: Option<i64>,
    ) -> Result<Vec<MessageData>, MessageStoreError>;
}

impl Put for MessageStore {
    fn put(
        &mut self,
        data: &MessageData,
        stream_name: &str,
        expected_version: Option<i64>,
    ) -> Result<MessageData, MessageStoreError> {
        put(&mut self.client, data, stream_name, expected_version)
    }

    fn put_many(
        &mut self,
        data: Vec<&MessageData>,
        stream_name: &str,
        expected_version: Option<i64>,
    ) -> Result<Vec<MessageData>, MessageStoreError> {
        put_many(&mut self.client, data, stream_name, expected_version)
    }
}

pub fn put_many<T: GenericClient>(
    client: &mut T,
    message_data: Vec<&MessageData>,
    stream_name: &str,
    expected_version: Option<i64>,
) -> Result<Vec<MessageData>, MessageStoreError> {
    let mut tx = client.transaction()?;
    let mut next = expected_version;
    let mut results: Vec<MessageData> = vec![];

    for data in message_data {
        let result = put(&mut tx, data, stream_name, next)?;
        results.push(result);

        if let Some(ver) = next {
            next = Some(ver + 1);
        }
    }

    tx.commit()?;

    Ok(results)
}

pub fn put<T: GenericClient>(
    client: &mut T,
    data: &MessageData,
    stream_name: &str,
    expected_version: Option<i64>,
) -> Result<MessageData, MessageStoreError> {
    let mut message = data.clone();

    if message.id.is_none() {
        message.id = Some(identity::uuid());
    }

    let id = message.id.as_ref().unwrap();

    let q = "SELECT write_message($1::varchar, $2::varchar, $3::varchar, $4::jsonb, $5::jsonb, $6::bigint);";

    let row = client.query_one(
        q,
        &[
            &id.to_string().as_str(),
            &String::from(stream_name),
            &data.message_type,
            &data.data,
            &data.metadata,
            &expected_version,
        ],
    );

    if let Err(ref e) = row {
        let msg = e.to_string();
        if msg.starts_with("ERROR: Wrong expected version") {
            return Err(MessageStoreError::ExpectedVersion(msg));
        }
    }

    message.position = row?.get(0);

    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::Put;
    use crate::message_store::errors::MessageStoreError;
    use crate::message_store::{controls, MessageData};
    use crate::stream_name;
    use crate::Json;
    use crate::Uuid;

    #[test]
    fn puts_message_data_into_stream_storage() {
        let mut store = controls::message_store();
        let client = &mut store.client;

        let mut data = controls::new_example();
        let stream_name = stream_name::controls::unique_example();

        let result = store.put(&mut data, &stream_name, Some(-1)).unwrap();

        assert_eq!(0, result.position.unwrap());
        assert!(result.id.is_some());

        let id = result.id.as_ref().unwrap();
        let result = store
            .client
            .query_one(
                "SELECT id, stream_name, type, data, metadata
                 FROM messages \
                 WHERE id = $1::uuid",
                &[id],
            )
            .unwrap();

        let written_id: Uuid = result.get(0);
        let written_stream_name: &str = result.get(1);
        let written_type: &str = result.get(2);
        let written_data: Json = result.get(3);
        let written_metadata: Json = result.get(4);

        assert_eq!(id, &written_id);
        assert_eq!(stream_name.as_str(), written_stream_name);
        assert_eq!(data.message_type.as_str(), written_type);
        assert_eq!(data.data, written_data);
        assert_eq!(data.metadata, written_metadata);
    }

    #[test]
    fn put_will_assign_a_new_id_if_one_is_not_provided() {
        let mut store = controls::message_store();
        let data = controls::new_example();
        let stream_name = stream_name::controls::unique_example();

        assert!(data.id.is_none());

        let result = store.put(&data, &stream_name, None).unwrap();

        assert!(result.id.is_some());
    }

    #[test]
    fn put_results_in_expected_version_error_when_stream_is_not_at_expected_version() {
        let mut store = controls::message_store();
        let data = controls::new_example();
        let stream_name = stream_name::controls::unique_example();

        let expected = format!(
            "ERROR: Wrong expected version: 10 (Stream: {}, Stream Version: -1)",
            stream_name
        );

        let result = store.put(&data, &stream_name, Some(10));

        assert!(result.is_err());

        if let Err(MessageStoreError::ExpectedVersion(e)) = result {
            assert_eq!(expected, e);
        }
    }

    #[test]
    fn put_many_will_put_many_datum_into_stream_storage() {
        let mut store = controls::message_store();
        let stream_name = stream_name::controls::unique_example();

        let data: Vec<MessageData> = (0..10).map(|_| controls::new_example()).collect();

        let results = store
            .put_many(data.iter().collect(), stream_name.as_str(), None)
            .unwrap();

        assert_eq!(10, results.len());

        for i in 0..results.len() {
            let result = results.get(i).unwrap();

            assert!(result.id.is_some());
            assert_eq!(i as i64, result.position.unwrap());
        }
    }
}
