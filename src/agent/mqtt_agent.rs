use anyhow::Result;
use rumqttc::{Client, Connection, MqttOptions, QoS};
use std::time::Duration;

const KEEP_ALIVE_SECS: u64 = 5;

pub(crate) struct MqttAgent {
    client: Client,
    topic: String,
}

impl MqttAgent {
    pub(super) fn new(host: String, port: u16, topic: String) -> Result<Self> {
        let mut opt = MqttOptions::new("replayman-mqtt", host, port);
        opt.set_keep_alive(Duration::from_secs(KEEP_ALIVE_SECS));

        let (client, connection) = Client::new(opt, 10);
        Self::spawn_notification_loop(connection);

        Ok(Self { client, topic })
    }

    pub(super) fn write(&mut self, log: String) -> Result<()> {
        self.client
            .publish(&self.topic, QoS::AtLeastOnce, false, log)?;
        Ok(())
    }

    fn spawn_notification_loop(mut connection: Connection) {
        std::thread::spawn(move || {
            for notification in connection.iter() {
                log::debug!("Notification = {:?}", notification);
            }
        });
    }
}
