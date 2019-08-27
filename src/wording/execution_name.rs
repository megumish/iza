pub struct ExecutionName {
    name_string: String,
}

impl From<String> for ExecutionName {
    fn from(name_string: String) -> Self {
        Self { name_string }
    }
}
