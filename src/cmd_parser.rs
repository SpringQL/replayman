use std::net::ToSocketAddrs;

use anyhow::{anyhow, Context, Result};
use chrono::DateTime;
use clap::Parser;
use springql_foreign_service::source::source_input::{
    timed_stream::{file_type::FileType, TimedStream},
    ForeignSourceInput,
};

use crate::destination::Destination;

// This file is part of https://github.com/SpringQL/replayman which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

/// Log agent to replay time-stamped log stream.
#[derive(Parser)]
#[clap(author = "Sho Nakatani <lay.sakura@gmail.com>")]
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

    pub(super) fn logs(&self) -> Result<ForeignSourceInput> {
        let log_file_type = match self.0.log_file_type.as_str() {
            "tsv" => Ok(FileType::Tsv),
            _ => Err(anyhow!(
                "unsupported log file type: {}",
                self.0.log_file_type
            )),
        }?;
        let timed_by = self.0.timed_by.to_string();
        let initial_timestamp = DateTime::parse_from_rfc3339(self.0.initial_timestamp.as_str())?;
        let log_file_path = self.0.log_file_path.as_str();

        let timed_stream =
            TimedStream::new(log_file_type, log_file_path, timed_by, initial_timestamp)?;

        Ok(ForeignSourceInput::new_timed_stream(timed_stream))
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
