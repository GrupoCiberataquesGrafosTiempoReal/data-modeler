mod config;
mod utils;
mod interface;

use anyhow::Result;
use utils::kafka;
use utils::db;
use std::time::{Instant, Duration};
use futures::StreamExt;
use rdkafka::message::BorrowedMessage;
use rdkafka::consumer::{Consumer, CommitMode};
use utils::processor::process_batch;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::from_environment()?;

    let consumer = kafka::create_consumer(&config)?;
    let client = db::new_client(&config).await?;

    db::ensure_ip_uniqueness_constraint(&client).await?;

    let mut batch: Vec<BorrowedMessage<'_>> = Vec::with_capacity(config.batch_size);
    let mut batch_start = Instant::now();

    let mut stream = consumer.stream();
    loop {
        tokio::select! {
            message_result = stream.next() => {
                if let Some(result) = message_result {
                    match result {
                        Ok(message) => {
                            batch.push(message);

                            if batch.len() >= config.batch_size {
                                process_batch(&consumer, &client, &mut batch).await?;
                                batch_start = Instant::now();
                            }
                        },
                        Err(e) => {
                            eprintln!("[Error] Message receiving error: {}", e);
                        }
                    }
                }
            }

            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                if batch_start.elapsed() >= config.batch_timeout {
                    if !batch.is_empty() {
                        process_batch(&consumer, &client, &mut batch).await?;
                    }

                    batch_start = Instant::now();
                }
            }

            _ = tokio::signal::ctrl_c() => {
                println!("Shutdown signal received. Committing offsets and exiting...");
                consumer.commit_consumer_state(CommitMode::Sync)?;
                break
            }
        }
    }

    Ok(())
}