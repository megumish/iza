#[derive(Clone, PartialEq)]
pub struct CredentialID {
    id_string: String,
}

impl ToString for CredentialID {
    fn to_string(&self) -> String {
        self.id_string.to_owned()
    }
}

impl From<String> for CredentialID {
    fn from(id_string: String) -> Self {
        Self { id_string }
    }
}
