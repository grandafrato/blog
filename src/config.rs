use lazy_static::lazy_static;
use std::net::SocketAddr;

pub struct Config {
    pub socket_addr: SocketAddr,
}

lazy_static! {
    pub static ref CONFIG: Config = Config {
        socket_addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
    };
}
