#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct YamlPackage {
    name: String,
}

impl YamlPackage {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name_of_yaml_package(&self) -> String {
        self.name.to_string()
    }
}
