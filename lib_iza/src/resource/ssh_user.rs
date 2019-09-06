pub struct SSHUser {
    value: String,
}

impl SSHUser {}

impl From<String> for SSHUser {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl ToString for SSHUser {
    fn to_string(&self) -> String {
        self.value.to_owned()
    }
}
