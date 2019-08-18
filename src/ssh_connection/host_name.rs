#[derive(Clone)]
pub struct HostName {
    name_string: String,
}

impl HostName {
    pub fn new(name_string: String) -> Self {
        Self { name_string }
    }
}

impl ToString for HostName {
    fn to_string(&self) -> String {
        self.name_string.clone()
    }
}
