#[derive(Clone)]
pub struct HostName {
    name_string: String,
}

impl HostName {
    pub fn new(name_string: String) -> Self {
        Self { name_string }
    }
}
