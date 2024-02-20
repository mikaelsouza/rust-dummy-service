use env_logger;
use envconfig::Envconfig;
use log;
mod config;
mod handlers;
mod model;
mod setup;
mod structs;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .init();

    log::info!("Starting the server");

    let env_config = config::Config::init_from_env().unwrap();
    let router = setup::define_routes();
    let address = setup::define_address();

    log::info!("Variable inside env_config: {:?}", env_config.env);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .expect("Unable to start server");
}
