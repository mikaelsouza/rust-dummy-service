use crate::structs::ModelParams;
use axum::extract::Query;
use ndarray::Array2;
use ort;

pub struct Model {
    pub session: ort::Session,
}

impl Model {
    pub fn new() -> Self {
        Model {
            session: ort::Session::builder()
                .unwrap()
                .with_execution_providers([ort::CPUExecutionProvider::default().build()])
                .unwrap()
                .with_model_from_file("model/logreg_iris.onnx")
                .unwrap(),
        }
    }

    pub fn run_model(&self, data: Query<ModelParams>) -> ort::SessionOutputs {
        let input_data = vec![[data.val1, data.val2, data.val3, data.val4]];
        let input_array = Array2::from(input_data);
        let input_tensor = ort::inputs![input_array].unwrap();
        self.session.run(input_tensor).unwrap()
    }
}
