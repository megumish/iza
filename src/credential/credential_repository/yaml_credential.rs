#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct YamlCredential {
    id: String,
    kind: String,
}

impl YamlCredential {
    pub fn new(id: String, kind: String) -> Self {
        Self { id, kind }
    }

    pub fn id_of_yaml_credential(&self) -> String {
        self.id.to_string()
    }

    pub fn kind_of_yaml_credential(&self) -> String {
        self.kind.to_string()
    }
}
