// Copyright (c) 2021 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

mod mqtt_agent;
mod tcp_agent;

use anyhow::Result;

use crate::destination::Destination;

use self::{mqtt_agent::MqttAgent, tcp_agent::TcpAgent};

pub(super) enum Agent {
    Tcp(TcpAgent),
    Mqtt(MqttAgent),
}

impl Agent {
    pub(super) fn new(dest: Destination) -> Result<Self> {
        match dest {
            Destination::Tcp(tcp_addr) => Ok(Self::Tcp(TcpAgent::new(tcp_addr)?)),
            Destination::Mqtt { host, port, topic } => {
                Ok(Self::Mqtt(MqttAgent::new(host, port, topic)?))
            }
        }
    }

    pub(super) fn write(&mut self, log: String) -> Result<()> {
        match self {
            Agent::Tcp(agent) => agent.write(log),
            Agent::Mqtt(agent) => agent.write(log),
        }
    }
}
