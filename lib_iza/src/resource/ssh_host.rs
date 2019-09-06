pub struct SSHHost {
    value: String,
}

impl SSHHost {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl From<String> for SSHHost {
    fn from(value: String) -> Self {
        Self { value }
    }
}
