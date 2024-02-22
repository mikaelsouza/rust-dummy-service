use crate::{handlers, model, structs};
use axum::routing::get;
use axum::Router;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, RwLock};

pub fn define_routes() -> Router {
    let app_state = structs::AppState {
        model: Arc::new(RwLock::new(model::Model::new())),
    };

    Router::new()
        .route("/", get(handlers::hello_world))
        .route("/name", get(handlers::hello_name))
        .route("/model", get(handlers::handle_model))
        .with_state(app_state)
}

pub fn define_address() -> SocketAddr {
    let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port = 3000;

    SocketAddr::new(ip, port)
}
