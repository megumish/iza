pub struct SSHHost {
    value: String,
}

impl SSHHost {}

impl From<String> for SSHHost {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl ToString for SSHHost {
    fn to_string(&self) -> String {
        self.value.to_owned()
    }
}
