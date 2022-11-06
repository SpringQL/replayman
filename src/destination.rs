// This file is part of https://github.com/SpringQL/replayman which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::net::SocketAddr;

#[derive(Eq, PartialEq, Debug)]
pub(super) enum Destination {
    Stdout,
    Tcp(SocketAddr),
    Mqtt {
        host: String,
        port: u16,
        topic: String,
    },
}
