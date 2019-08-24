#[derive(Debug, Clone, PartialEq)]
pub struct PackageID {
    id_string: String,
}

impl From<String> for PackageID {
    fn from(id_string: String) -> Self {
        Self { id_string }
    }
}

impl ToString for PackageID {
    fn to_string(&self) -> String {
        self.id_string.clone()
    }
}
