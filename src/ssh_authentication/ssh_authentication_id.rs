pub struct SSHAuthenticationID {
    id_string: String,
}

impl SSHAuthenticationID {
    pub fn new() -> Self {
        let id_string = uuid::Uuid::new_v4().to_hyphenated().to_string();
        Self { id_string }
    }
}

impl From<String> for SSHAuthenticationID {
    fn from(id_string: String) -> Self {
        Self { id_string }
    }
}
