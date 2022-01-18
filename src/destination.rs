// Copyright (c) 2021-2022 TOYOTA MOTOR CORPORATION. Licensed under MIT OR Apache-2.0.

use std::net::SocketAddr;

#[derive(Eq, PartialEq, Debug)]
pub(super) enum Destination {
    Tcp(SocketAddr),
    Mqtt {
        host: String,
        port: u16,
        topic: String,
    },
}
