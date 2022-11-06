// This file is part of https://github.com/SpringQL/replayman which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

mod mqtt_agent;
mod stdout_agent;
mod tcp_agent;

use anyhow::Result;

use crate::{agent::stdout_agent::StdoutAgent, destination::Destination};

use self::{mqtt_agent::MqttAgent, tcp_agent::TcpAgent};

pub(super) enum Agent {
    Stdout(StdoutAgent),
    Tcp(TcpAgent),
    Mqtt(MqttAgent),
}

impl Agent {
    pub(super) fn new(dest: Destination) -> Result<Self> {
        match dest {
            Destination::Stdout => Ok(Self::Stdout(StdoutAgent::default())),
            Destination::Tcp(tcp_addr) => Ok(Self::Tcp(TcpAgent::new(tcp_addr)?)),
            Destination::Mqtt { host, port, topic } => {
                Ok(Self::Mqtt(MqttAgent::new(host, port, topic)?))
            }
        }
    }

    pub(super) fn write(&mut self, log: String) -> Result<()> {
        match self {
            Agent::Stdout(agent) => agent.write(&log),
            Agent::Tcp(agent) => agent.write(log),
            Agent::Mqtt(agent) => agent.write(log),
        }
    }
}
