use std::net::ToSocketAddrs;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use time::OffsetDateTime;

use crate::{
    destination::Destination,
    timed_stream::{file_type::FileType, TimedStream},
};

// This file is part of https://github.com/SpringQL/replayman which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

/// Log agent to replay time-stamped log stream.
#[derive(Parser)]
#[clap(author = "Sho Nakatani <lay.sakura@gmail.com>", version)]
struct Opts {
    /// Either of: tsv
    #[clap(long, default_value = "tsv")]
    log_file_type: String,

    /// Field name of log file to be used as timestamp.
    /// Field value must be formatted in RFC-3339 (e.g. 2006-04-13T14:12:53.4242+05:30)
    #[clap(long)]
    timed_by: String,

    /// (Virtual) timestamp to start replay from.
    /// Must be formatted in RFC-3339 (e.g. 2006-04-13T14:12:53.4242+05:30)
    #[clap(long)]
    initial_timestamp: String,

    /// Playback speed.
    /// If you set it to 0.5, for example, the internal virtual timer ticks at half the speed compared to wall-clock timers.
    #[clap(long, default_value = "1")]
    speed: String,

    /// TCP address:port to write logs to.
    ///
    /// (e.g. --dest-tcp 'localhost:19870')
    #[clap(long)]
    dest_tcp: Option<String>,

    /// MQTT address:port to publish logs to.
    ///
    /// (e.g. --dest-mqtt 'localhost:19870' --dest-mqtt-topic 'your/topic')
    #[clap(long)]
    dest_mqtt: Option<String>,
    /// MQTT topic.
    #[clap(long)]
    dest_mqtt_topic: Option<String>,

    /// Log file to replay
    log_file_path: String,
}

pub(super) struct CmdParser(Opts);

impl CmdParser {
    pub(super) fn new() -> Self {
        let opts: Opts = Opts::parse();
        Self(opts)
    }

    pub(super) fn logs(&self) -> Result<TimedStream> {
        let log_file_type = match self.0.log_file_type.as_str() {
            "tsv" => Ok(FileType::Tsv),
            _ => Err(anyhow!(
                "unsupported log file type: {}",
                self.0.log_file_type
            )),
        }?;
        let timed_by = self.0.timed_by.to_string();
        let initial_timestamp = OffsetDateTime::parse(
            self.0.initial_timestamp.as_str(),
            &time::format_description::well_known::Rfc3339,
        )?;
        let speed = self.0.speed.parse::<f32>()?;
        let log_file_path = self.0.log_file_path.as_str();

        TimedStream::new(
            log_file_type,
            log_file_path,
            timed_by,
            initial_timestamp,
            speed,
        )
    }

    pub(super) fn dest(&self) -> Result<Destination> {
        match (&self.0.dest_tcp, &self.0.dest_mqtt, &self.0.dest_mqtt_topic) {
            (Some(tcp_addr), None, None) => {
                let addr = tcp_addr
                    .to_socket_addrs()?
                    .next()
                    .context("empty address?")?;
                Ok(Destination::Tcp(addr))
            }
            (None, Some(mqtt_addr), Some(mqtt_topic)) => {
                let errmsg = || format!("failed to parse MQTT address: {}", mqtt_addr);

                let mut addr = mqtt_addr.split(':');

                let host = addr.next().with_context(errmsg)?;
                let port = addr.next().with_context(errmsg)?;
                let port: u16 = port.parse().with_context(errmsg)?;

                Ok(Destination::Mqtt {
                    host: host.to_string(),
                    port,
                    topic: mqtt_topic.to_string(),
                })
            }
            _ => Err(anyhow!("A `--dest-*` option is required")),
        }
    }
}
