use crate::message_store::{MessageData, MessageStoreError};
use std::io::Write;

pub fn bulk_insert(
    client: &mut postgres::Client,
    data: Vec<&MessageData>,
) -> Result<usize, MessageStoreError> {
    let q = "COPY messages(stream_name, type, position, data, metadata) \
             FROM stdin";
    let mut writer = client.copy_in(q)?;

    for message in data.iter() {
        writer.write_fmt(format_args!(
            "{}\t{}\t{}\t{}\t{}\n",
            option(&message.stream_name)?,
            message.message_type,
            option(&message.position)?,
            message.data.as_str().unwrap(),
            message.metadata.as_str().unwrap(),
        ))?;
    }

    writer.finish()?;

    Ok(data.len())
}

fn option<T>(field: &Option<T>) -> Result<&T, MessageStoreError> {
    match field {
        Some(value) => Ok(value),
        None => Err(MessageStoreError::MissingField),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message_store::{controls, Get};
    use crate::{db, stream_name, Utc};

    #[test]
    fn bulk_inserts_message_data() {
        let mut store = controls::message_store();
        let category = stream_name::controls::unique_category();
        let data: Vec<MessageData> = (0..100)
            .map(|i| {
                let mut message = controls::example();
                message.position = Some(i);
                message.stream_name = Some(category.clone());
                message.time = Some(Utc::now());
                message
            })
            .collect();

        let mut client = db::build();

        bulk_insert(&mut client, data.iter().collect()).unwrap();

        let results = store.get(&category, None).unwrap();

        println!("category {}", category);

        assert_eq!(100, results.len());
    }
}
