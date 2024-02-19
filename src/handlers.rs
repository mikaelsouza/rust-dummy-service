use crate::model;
use axum::extract::Query;
use ndarray::Array2;
use ort;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Name {
    name: String,
}

#[derive(Deserialize)]
pub struct Model {
    val1: f32,
    val2: f32,
    val3: f32,
    val4: f32,
}

pub async fn hello_world() -> String {
    format!("Hello, World!\n")
}

pub async fn hello_name(query: Query<Name>) -> String {
    format!("Hello, {}\n", query.name)
}

pub async fn model(query: Query<Model>) -> String {
    let model = model::load_model();
    let input_data = vec![[query.val1, query.val2, query.val3, query.val4]];
    let input_array = Array2::from(input_data);
    let input_tensor = ort::inputs![input_array].unwrap();
    let outputs = model.run(input_tensor).unwrap();
    let result: (Vec<i64>, &[i64]) = outputs["output_label"].extract_raw_tensor().unwrap();
    format!("{:?}", result.1)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_model_0() {
        let inputs = Model {
            val1: 5.1,
            val2: 3.5,
            val3: 1.4,
            val4: 0.2,
        };

        let query = Query(inputs);
        let results = model(query).await;
        assert_eq!(results, String::from("[0]"));
    }
    #[tokio::test]
    async fn test_model_1() {
        let inputs = Model {
            val1: 7.0,
            val2: 3.2,
            val3: 4.7,
            val4: 1.4,
        };

        let query = Query(inputs);
        let results = model(query).await;
        assert_eq!(results, String::from("[1]"));
    }

    #[tokio::test]
    async fn test_model_2() {
        let inputs = Model {
            val1: 6.3,
            val2: 3.3,
            val3: 6.0,
            val4: 2.5,
        };

        let query = Query(inputs);
        let results = model(query).await;
        assert_eq!(results, String::from("[2]"));
    }
}
