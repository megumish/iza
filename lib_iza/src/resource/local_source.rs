pub struct LocalSource {
    path_string: String,
}

impl LocalSource {
    pub fn new(path_string: String) -> Self {
        Self { path_string }
    }
}
