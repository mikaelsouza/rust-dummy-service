use axum::{extract::Query, routing::get, Router};
use envconfig::Envconfig;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

#[derive(Debug)]
// Configuration code for environment
enum Environment {
    PROD,
    DEV,
}

impl FromStr for Environment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PROD" => Ok(Self::PROD),
            "DEV" => Ok(Self::DEV),
            //_ => Err(format!("Invalid environment: {}", s)),
            _ => {
                println!("Environment from ENV different from [PROD|DEV]. Env value: {}. Defaulting to DEV.", s);
                Ok(Self::DEV)
            }
        }
    }
}

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "ENVIRONMENT", default = "DEV")]
    pub env: Environment,
}

// Code for functions that will triggered by API call
#[derive(Deserialize)]
struct Name {
    name: String,
}

async fn hello_world() -> String {
    return "Hello, World!\n".to_string();
}

async fn hello_name(query: Query<Name>) -> String {
    return format!("Hello, {}\n", query.name);
}

// Route definition
fn define_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/name", get(hello_name))
}

fn define_address() -> SocketAddr {
    let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let port = 3000;

    SocketAddr::new(ip, port)
}

#[tokio::main]
async fn main() {
    let env_config = Config::init_from_env().unwrap();

    println!("Variable inside env_config: {:?}", env_config.env);

    let router = define_routes();
    let address = define_address();

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .expect("Unable to start server");
}
