use anyhow::{anyhow, Result};
use chrono::DateTime;
use clap::Parser;
use springql_foreign_service::source::source_input::{
    timed_stream::{file_type::FileType, TimedStream},
    ForeignSourceInput,
};

// Copyright (c) 2021 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

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

    /// Log file to replay
    log_file_path: String,
}

pub(super) struct CmdParser(Opts);

impl CmdParser {
    pub(super) fn new() -> Self {
        let opts: Opts = Opts::parse();
        Self(opts)
    }

    pub(super) fn foreign_source_input(&self) -> Result<ForeignSourceInput> {
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
}
