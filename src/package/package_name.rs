#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PackageName {
    name_string: String,
}

impl PackageName {
    pub fn new(name_string: String) -> PackageName {
        Self { name_string }
    }
}

impl ToString for PackageName {
    fn to_string(&self) -> String {
        self.name_string.to_owned()
    }
}

impl From<String> for PackageName {
    fn from(name_string: String) -> Self {
        Self { name_string }
    }
}
