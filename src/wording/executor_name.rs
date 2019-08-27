pub struct ExecutorName {
    name_string: String,
}

impl From<String> for ExecutorName {
    fn from(name_string: String) -> Self {
        Self { name_string }
    }
}
