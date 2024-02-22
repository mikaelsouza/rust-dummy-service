use crate::model::Model;
use serde::Deserialize;
use std::sync::{Arc, RwLock};

#[derive(Deserialize)]
pub struct ModelParams {
    pub val1: f32,
    pub val2: f32,
    pub val3: f32,
    pub val4: f32,
}

#[derive(Deserialize)]
pub struct Name {
    pub name: String,
}

#[derive(Clone)]
pub struct AppState {
    pub model: Arc<RwLock<Model>>,
}
