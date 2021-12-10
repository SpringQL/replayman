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
