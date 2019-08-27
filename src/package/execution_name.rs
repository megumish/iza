#[derive(Debug, PartialEq, Clone)]
pub struct ExecutionName {
    name_string: String,
}

impl ToString for ExecutionName {
    fn to_string(&self) -> String {
        self.name_string.to_owned()
    }
}

impl From<String> for ExecutionName {
    fn from(name_string: String) -> Self {
        Self { name_string }
    }
}
