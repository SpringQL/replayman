use anyhow::{Context, Result};
use std::{
    io::{BufWriter, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

const CONNECT_TIMEOUT_SECS: u64 = 1;
const WRITE_TIMEOUT_MSECS: u64 = 100;

pub(crate) struct TcpAgent {
    tcp_stream_writer: BufWriter<TcpStream>,
}

impl TcpAgent {
    pub(super) fn new(dest_addr: SocketAddr) -> Result<Self> {
        let tcp_stream =
            TcpStream::connect_timeout(&dest_addr, Duration::from_secs(CONNECT_TIMEOUT_SECS))
                .context("failed to connect to remote host")?;
        tcp_stream
            .set_write_timeout(Some(Duration::from_millis(WRITE_TIMEOUT_MSECS)))
            .context("failed to set timeout to remote host")?;

        let tcp_stream_writer = BufWriter::new(tcp_stream);
        Ok(Self { tcp_stream_writer })
    }

    pub(super) fn write(&mut self, mut log: String) -> Result<()> {
        log.push('\n');

        self.tcp_stream_writer
            .write_all(log.as_bytes())
            .with_context(|| format!("failed to write log line to remote: {}", log))?;
        self.tcp_stream_writer
            .flush()
            .with_context(|| format!("failed to flush log line: {}", log))?;

        Ok(())
    }
}
