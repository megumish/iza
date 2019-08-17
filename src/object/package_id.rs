pub struct PackageID {
    id_string: String,
}

impl From<String> for PackageID {
    fn from(id_string: String) -> Self {
        Self { id_string }
    }
}
