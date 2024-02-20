use axum::extract::Query;
use ndarray::Array2;
use ort;

use crate::structs::ModelParams;

pub struct Model {}

impl Model {
    fn get_session() -> ort::Session {
        ort::Session::builder()
            .unwrap()
            .with_model_from_file("model/logreg_iris.onnx")
            .unwrap()
    }

    pub fn run_model(data: Query<ModelParams>) -> (Vec<i64>, Vec<i64>) {
        let session = Model::get_session();
        let input_data = vec![[data.val1, data.val2, data.val3, data.val4]];
        let input_array = Array2::from(input_data);
        let input_tensor = ort::inputs![input_array].unwrap();
        let outputs = session.run(input_tensor).unwrap();
        let results: (Vec<i64>, &[i64]) = outputs["output_label"].extract_raw_tensor().unwrap();
        (results.0, results.1.to_vec())
    }
}
