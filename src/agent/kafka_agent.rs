// This file is part of https://github.com/SpringQL/replayman which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::time::Duration;

use anyhow::{anyhow, Result};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

pub(crate) struct KafkaAgent {
    producer: FutureProducer,
    topic: String,
}

impl KafkaAgent {
    pub(super) fn new(bootstrap_servers: String, topic: String) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("failed to create Kafka producer");

        Self { producer, topic }
    }

    /// Publish a message (log) with a random key.
    pub(super) async fn write(&self, log: String) -> Result<()> {
        self.producer
            .send(
                FutureRecord::<[u8; 8], _>::to(&self.topic).payload(&log),
                Duration::from_secs(0),
            )
            .await
            .map_err(|(e, _)| anyhow!(e))
            .map(|_| {
                log::debug!("send a message to Kafka");
            })
    }
}
