use axum::extract::Query;
use ndarray::Array2;
use ort;

use crate::structs::ModelParams;

pub struct Model {
    pub model: Option<ort::Session>,
}

impl Model {
    pub fn load_model(&mut self) {
        self.model = Some(
            ort::Session::builder()
                .unwrap()
                .with_execution_providers([ort::CoreMLExecutionProvider::default().build()])
                .unwrap()
                .with_model_from_file("model/logreg_iris.onnx")
                .unwrap(),
        );
    }

    pub fn run_model(&mut self, data: Query<ModelParams>) -> (Vec<i64>, Vec<i64>) {
        self.load_model();
        let input_data = vec![[data.val1, data.val2, data.val3, data.val4]];
        let input_array = Array2::from(input_data);
        let input_tensor = ort::inputs![input_array].unwrap();
        let output = self.model.as_ref().unwrap().run(input_tensor).unwrap();
        let (i, j): (Vec<i64>, &[i64]) = output["output_label"].extract_raw_tensor().unwrap();
        let j = j.to_owned();
        (i, j)
    }
}
