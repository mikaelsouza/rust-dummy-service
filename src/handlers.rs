use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Name {
    name: String,
}

pub async fn hello_world() -> String {
    format!("Hello, World!\n")
}

pub async fn hello_name(query: Query<Name>) -> String {
    format!("Hello, {}\n", query.name)
}
