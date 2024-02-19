use ort;

pub fn load_model() -> ort::Session {
    let model = ort::Session::builder()
        .unwrap()
        .with_execution_providers([ort::CPUExecutionProvider::default().build()])
        .unwrap()
        .with_model_from_file("model/logreg_iris.onnx")
        .unwrap();
    model
}
