pub struct SSHConnectionID {
    id_string: String,
}

impl SSHConnectionID {
    pub fn new() -> Self {
        let id_string = uuid::Uuid::new_v4().to_hyphenated().to_string();
        Self { id_string }
    }
}

impl ToString for SSHConnectionID {
    fn to_string(&self) -> String {
        self.id_string.to_owned()
    }
}

impl From<String> for SSHConnectionID {
    fn from(id_string: String) -> Self {
        Self { id_string }
    }
}
