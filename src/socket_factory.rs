use std::net::{Ipv4Addr, SocketAddr};
use crate::STATIC_CONFIG;

pub fn of() -> SocketAddr{
    return SocketAddr::new(
        std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        STATIC_CONFIG
            .lock()
            .unwrap()
            .get("port")
            .unwrap()
            .parse::<u16>()
            .unwrap(),
    );

}