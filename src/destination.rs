use std::net::SocketAddr;

#[derive(Eq, PartialEq, Debug)]
pub(super) enum Destination {
    Tcp(SocketAddr),
}
