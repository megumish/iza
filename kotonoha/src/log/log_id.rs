pub struct LogID {
    id_string: String,
}

impl LogID {
    pub fn new() -> Self {
        let id_string = uuid::Uuid::new_v4().to_simple().to_string();

        Self { id_string }
    }
}

impl std::fmt::Display for LogID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.id_string)
    }
}
