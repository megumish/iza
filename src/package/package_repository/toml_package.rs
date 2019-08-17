#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct TomlPackage {
    name: String,
}

impl TomlPackage {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name_of_toml_package(&self) -> String {
        self.name.to_string()
    }
}
