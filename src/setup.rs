use crate::handlers;
use axum::{routing::get, Router};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub fn define_routes() -> Router {
    Router::new()
        .route("/", get(handlers::hello_world))
        .route("/name", get(handlers::hello_name))
}

pub fn define_address() -> SocketAddr {
    let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port = 3000;

    SocketAddr::new(ip, port)
}
