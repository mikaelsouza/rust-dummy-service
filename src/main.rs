use envconfig::Envconfig;

mod config;
mod handlers;
mod setup;

#[tokio::main]
async fn main() {
    let env_config = config::Config::init_from_env().unwrap();
    let router = setup::define_routes();
    let address = setup::define_address();

    println!("Variable inside env_config: {:?}", env_config.env);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .expect("Unable to start server");
}
