use std::io::Write;

use crate::message_store::MessageData;
use crate::Error;

pub fn bulk_insert(client: &mut postgres::Client, data: Vec<&MessageData>) -> Result<usize, Error> {
    let q = "COPY messages(stream_name, type, position, data, metadata) \
             FROM stdin";
    let mut writer = client.copy_in(q)?;

    for message in data.iter() {
        writer.write_fmt(format_args!(
            "{}\t{}\t{}\t{}\t{}\n",
            option(&message.stream_name)?,
            message.message_type,
            option(&message.position)?,
            message.data,
            message.metadata,
        ))?;
    }

    writer.finish()?;

    Ok(data.len())
}

fn option<T>(field: &Option<T>) -> Result<&T, Error> {
    match field {
        Some(value) => Ok(value),
        None => Err(Error::MissingField),
    }
}

#[cfg(test)]
mod tests {
    use crate::message_store::{controls, Get};
    use crate::{db, stream_name, Utc};

    use super::*;

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
