#[derive(Debug, PartialEq, Clone)]
pub struct ExecutionID {
    id_string: String,
}

impl ToString for ExecutionID {
    fn to_string(&self) -> String {
        self.id_string.to_owned()
    }
}

impl From<String> for ExecutionID {
    fn from(id_string: String) -> Self {
        Self { id_string }
    }
}
