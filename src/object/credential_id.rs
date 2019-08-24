#[derive(Debug, Clone)]
pub struct CredentialID {
    id_string: String,
}

impl From<String> for CredentialID {
    fn from(id_string: String) -> Self {
        Self { id_string }
    }
}

impl ToString for CredentialID {
    fn to_string(&self) -> String {
        self.id_string.clone()
    }
}
