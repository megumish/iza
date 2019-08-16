pub struct LocalPath {
    path_string: String,
}

impl From<String> for LocalPath {
    fn from(path_string: String) -> Self {
        Self { path_string }
    }
}
