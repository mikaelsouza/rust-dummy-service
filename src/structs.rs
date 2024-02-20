use serde::Deserialize;

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
