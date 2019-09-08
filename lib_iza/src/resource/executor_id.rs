#[derive(Clone, Serialize)]
pub struct ExecutorID {
    id_string: String,
}

impl ExecutorID {
    pub fn new() -> Self {
        let id_string = uuid::Uuid::new_v4().to_simple().to_string();
        Self { id_string }
    }
}
