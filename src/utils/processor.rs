use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
use rdkafka::consumer::{Consumer, StreamConsumer, CommitMode};
use neo4rs::Graph;
use serde_json::Value;
use std::time::Instant;
use anyhow::{anyhow, Result, Error};
use futures::future::join_all;
use crate::utils::db::model_query;
use crate::interface::summary::{create_summary, print_summary};

pub async fn process_batch(
    consumer: &StreamConsumer,
    client: &Graph,
    batch: &mut Vec<BorrowedMessage<'_>>,
) -> Result<()> {
    let start = Instant::now();

    let futures = batch.iter().map(|message| {
        let client = client.clone();

        async move {
            let payload = message.payload()
                .map(|bytes| String::from_utf8_lossy(bytes).to_string())
                .unwrap_or_default();

            let event: Value = serde_json::from_str(&payload).unwrap();

            let query = model_query(event);

            client.run(query).await.map_err(|err| anyhow!("Neo4j query run error: {}", err))?;
        
            Ok::<(), Error>(())
        }
    });

    let results = join_all(futures).await;

    let mut ok_count = 0;
    let mut err_count = 0;

    for result in results {
        match result {
            Ok(_) => ok_count += 1,
            Err(err) => {
                err_count += 1;
                eprintln!("[Error] Processing error: {}", err);
            }
        }
    }

    if let Some(last_message) = batch.last() {
        consumer.commit_message(last_message, CommitMode::Sync)?;
    }

    let batch_summary = create_summary(
        batch.len(),
        ok_count,
        err_count,
        start.elapsed().as_millis(),
    );
    print_summary(batch_summary);

    batch.clear();

    Ok(())
}