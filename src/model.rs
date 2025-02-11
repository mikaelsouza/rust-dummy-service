use crate::structs::ModelParams;
use axum::extract::Query;
use ndarray::Array2;
use ort::session::{Session, SessionOutputs};
use ort::execution_providers::CPUExecutionProvider;
pub struct Model {
    pub session: Session,
}

impl Model {
    pub fn new() -> Self {
        Model {
            session: Session::builder()
                .unwrap()
                .with_execution_providers([CPUExecutionProvider::default().build()])
                .unwrap()
                .commit_from_file("model/logreg_iris.onnx")
                .unwrap(),
        }
    }

    pub fn run_model(&self, data: Query<ModelParams>) -> SessionOutputs {
        let input_data = vec![[data.val1, data.val2, data.val3, data.val4]];
        let input_array = Array2::from(input_data);
        let input_tensor = ort::inputs![input_array].unwrap();
        self.session.run(input_tensor).unwrap()
    }
}
