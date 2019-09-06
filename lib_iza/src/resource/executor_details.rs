use std::collections::HashMap;

#[derive(Serialize)]
pub struct ExecutorDetails {
    details: HashMap<&'static str, String>,
}

impl ExecutorDetails {
    pub fn get<'a>(&self, key: &'static str) -> Option<&String> {
        self.details.get(key)
    }
}

#[cfg(test)]
impl From<HashMap<&'static str, String>> for ExecutorDetails {
    fn from(details: HashMap<&'static str, String>) -> Self {
        Self { details }
    }
}
