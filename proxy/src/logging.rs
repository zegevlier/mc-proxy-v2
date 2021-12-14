use std::{
    fs::File,
    io::{prelude::*, LineWriter},
    sync::Arc,
    time::SystemTime,
};

use serde::Serialize;

use crate::parsable::Parsable;

pub type LogQueue = deadqueue::unlimited::Queue<Box<dyn Parsable + Send + Sync>>;

#[derive(Serialize)]
struct LogShape<T>
where
    T: erased_serde::Serialize + Sized,
{
    timestamp: u128,
    r#type: String,
    value: T,
}

pub async fn logger(filename: &str, log_queue: Arc<LogQueue>) -> std::io::Result<()> {
    let file = File::create(filename).unwrap();
    let mut file = LineWriter::new(file);
    loop {
        let message = log_queue.pop().await;
        file.write_all(
            serde_json::to_string(&LogShape {
                timestamp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
                r#type: "Packet".to_string(),
                value: message,
            })
            .unwrap()
            .as_bytes(),
        )
        .unwrap();
        file.write_all(b"\n").unwrap();
    }
}
