pub struct WorkingDirectory {
    directory_string: String,
}

impl WorkingDirectory {
    pub fn new(directory_string: String) -> Self {
        Self { directory_string }
    }
}

impl ToString for WorkingDirectory {
    fn to_string(&self) -> String {
        self.directory_string.to_owned()
    }
}

impl From<String> for WorkingDirectory {
    fn from(directory_string: String) -> Self {
        Self { directory_string }
    }
}
