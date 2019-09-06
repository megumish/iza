pub struct SSHUser {
    value: String,
}

impl SSHUser {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl From<String> for SSHUser {
    fn from(value: String) -> Self {
        Self { value }
    }
}
