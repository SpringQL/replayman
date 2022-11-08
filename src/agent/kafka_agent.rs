// This file is part of https://github.com/SpringQL/replayman which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::{sync::Arc, time::Duration};

use anyhow::Result;
use rdkafka::{
    producer::{BaseProducer, BaseRecord},
    ClientConfig,
};

pub(crate) struct KafkaAgent {
    producer: Arc<BaseProducer>,
    topic: String,
}

impl KafkaAgent {
    pub(super) fn new(bootstrap_servers: String, topic: String) -> Result<Self> {
        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)
            .create()?;
        let producer = Arc::new(producer);

        Self::spawn_poll_loop(producer.clone());

        Ok(Self { producer, topic })
    }

    /// Publish a message (log) with a random key.
    /// This is a non-blocking function.
    pub(super) fn write(&self, log: String) -> Result<()> {
        self.producer
            .send(BaseRecord::<[u8], _, _>::to(&self.topic).payload(&log))
            .map_err(|(e, _)| e)?;

        log::debug!("send a message to Kafka");
        Ok(())
    }

    /// Poll at regular intervals to process all the asynchronous delivery events.
    fn spawn_poll_loop(producer: Arc<BaseProducer>) {
        std::thread::spawn(move || loop {
            let _ = producer.poll(Duration::from_millis(100));
            std::thread::sleep(Duration::from_millis(1));
        });
    }
}
