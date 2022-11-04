// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

pub mod file_type;

mod file_parser;
mod timer;

use std::{path::Path, thread, time::Duration};

use anyhow::{Context, Result};
use time::{format_description::well_known, OffsetDateTime};

use crate::timed_stream::{file_parser::FileParser, file_type::FileType, timer::Timer};

const SLEEP_DURATION: Duration = Duration::from_micros(10);

/// Open a file and read lines with a timestamp fields, then generates lines with past timestamp compared to "now".
///
/// If current line's timestamp is newer than "now", `Iterator::next()` blocks (with sleep).
///
/// If lines in a file is not ordered by timestamp, Timed Stream just generates `line_with_newer_timestamp -> line_with_older_timestamp` in consecutive iterations.
#[derive(Debug)]
pub struct TimedStream {
    timestamp_field: String,
    timer: Timer,
    file_parser: FileParser,
}

impl TimedStream {
    pub fn new<P: AsRef<Path>>(
        file_type: FileType,
        file_path: P,
        timestamp_field: String,
        virt_initial_datetime: OffsetDateTime,
        speed: f32,
    ) -> Result<Self> {
        let file_parser = FileParser::new(file_type, file_path)?;
        let timer = Timer::new(virt_initial_datetime, speed);
        Ok(Self {
            timestamp_field,
            timer,
            file_parser,
        })
    }
}

impl Iterator for TimedStream {
    type Item = Result<serde_json::Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.file_parser.next().map(|res_json| {
            let json = res_json?;

            let timestamp_s = json.get(&self.timestamp_field).with_context(|| {
                format!(
                    r#"timestamp field "{}" not found in line: {}"#,
                    self.timestamp_field, json
                )
            })?.as_str().with_context(|| {
                format!(
                    r#"timestamp field "{}" is a string in line: {}"#,
                    self.timestamp_field, json
                )
            })?;
            let timestamp =
                OffsetDateTime::parse(timestamp_s, &well_known::Rfc3339)
                    .with_context(||
                        format!(
                            r#"timestamp field "{}" is not in RFC 3339 format. Correct example: "1996-12-19T16:39:57-08:00""#,
                            timestamp_s
                        )
                    )?;

            while self.timer.virt_current_datetime() < timestamp {
                thread::sleep(SLEEP_DURATION);
            }

            Ok(json)
        })
    }
}
