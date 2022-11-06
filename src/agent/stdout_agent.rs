// This file is part of https://github.com/SpringQL/replayman which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use anyhow::Result;

#[derive(Debug, Default)]
pub(crate) struct StdoutAgent {}

impl StdoutAgent {
    pub(super) fn write(&mut self, log: &str) -> Result<()> {
        println!("{}", log);
        Ok(())
    }
}
