#[derive(Debug, Clone)]
pub struct RemotePath {
    path_string: String,
}

impl From<String> for RemotePath {
    fn from(path_string: String) -> Self {
        Self { path_string }
    }
}

impl ToString for RemotePath {
    fn to_string(&self) -> String {
        self.path_string.clone()
    }
}
