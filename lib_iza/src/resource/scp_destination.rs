pub struct SCPDestination {
    path_string: String,
}

impl SCPDestination {
    pub fn new(path_string: String) -> Self {
        Self { path_string }
    }
}
