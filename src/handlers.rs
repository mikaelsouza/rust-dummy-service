use crate::{
    model::Model,
    structs::{ModelParams, Name},
};
use axum::extract::Query;

pub async fn hello_world() -> String {
    format!("Hello, World!\n")
}

pub async fn hello_name(query: Query<Name>) -> String {
    format!("Hello, {}\n", query.name)
}

pub async fn handle_model(query: Query<ModelParams>) -> String {
    let model = Model::new();
    let outputs = model.run_model(query);
    let results: (Vec<i64>, &[i64]) = outputs["output_label"].extract_raw_tensor().unwrap();
    format!("{:?}", results.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_hello_world() {
        let results = hello_world().await;
        assert_eq!(results, String::from("Hello, World!\n"));
    }

    #[tokio::test]
    async fn test_hello_name() {
        let query = Name {
            name: String::from("John"),
        };
        let results = hello_name(Query(query)).await;
        assert_eq!(results, String::from("Hello, John\n"));
    }

    #[tokio::test]
    async fn test_model_0() {
        let inputs = ModelParams {
            val1: 5.1,
            val2: 3.5,
            val3: 1.4,
            val4: 0.2,
        };
        let query = Query(inputs);
        let results = handle_model(query).await;
        assert_eq!(results, String::from("[0]"));
    }
    #[tokio::test]
    async fn test_model_1() {
        let inputs = ModelParams {
            val1: 7.0,
            val2: 3.2,
            val3: 4.7,
            val4: 1.4,
        };

        let query = Query(inputs);
        let results = handle_model(query).await;
        assert_eq!(results, String::from("[1]"));
    }

    #[tokio::test]
    async fn test_model_2() {
        let inputs = ModelParams {
            val1: 6.3,
            val2: 3.3,
            val3: 6.0,
            val4: 2.5,
        };

        let query = Query(inputs);
        let results = handle_model(query).await;
        assert_eq!(results, String::from("[2]"));
    }
}
