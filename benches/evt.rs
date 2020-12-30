use criterion::{black_box, criterion_group, criterion_main, Criterion};
use evt::message_store::{Get, MessageData, Put};
use evt::{message_store, stream_name};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("put 1", |b| {
        let mut store = message_store::controls::message_store();
        let stream = stream_name::controls::unique_example();
        let data = message_store::controls::new_example();

        b.iter(|| store.put(&data, &stream, None))
    });

    c.bench_function("put 100", |b| {
        let mut store = message_store::controls::message_store();
        let stream = stream_name::controls::unique_example();
        let data: Vec<MessageData> = (0..100)
            .map(|_| message_store::controls::new_example())
            .collect();

        b.iter(|| store.put_many(data.iter().collect(), &stream, None))
    });

    c.bench_function("get 1", |b| {
        let mut store = message_store::controls::message_store();
        store.settings.batch_size = Some(1);
        let stream = stream_name::controls::unique_example();
        let data = message_store::controls::new_example();
        &store.put(&data, &stream, None).unwrap();

        b.iter(|| store.get(&stream, None))
    });

    c.bench_function("get 100", |b| {
        let mut store = message_store::controls::message_store();
        store.settings.batch_size = Some(100);
        let stream = stream_name::controls::unique_example();
        let data: Vec<MessageData> = (0..100)
            .map(|i| {
                let mut msg = message_store::controls::new_example();
                msg.position = Some(i);
                msg.stream_name = Some(stream.clone());
                msg
            })
            .collect();

        message_store::tools::bulk_insert(&mut store.client, data.iter().collect()).unwrap();

        b.iter(|| store.get(&stream, None))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
