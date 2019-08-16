pub struct RemotePath {
    path_string: String,
}

impl From<String> for RemotePath {
    fn from(path_string: String) -> Self {
        Self { path_string }
    }
}
