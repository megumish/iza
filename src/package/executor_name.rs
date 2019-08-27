#[derive(Clone, PartialEq, Debug)]
pub struct ExecutorName {
    name_string: String,
}

impl ToString for ExecutorName {
    fn to_string(&self) -> String {
        self.name_string.to_owned()
    }
}

impl From<String> for ExecutorName {
    fn from(name_string: String) -> Self {
        Self { name_string }
    }
}
