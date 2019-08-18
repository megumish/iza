#[derive(Clone)]
pub struct ObjectID {
    id_string: String,
}

impl ObjectID {
    pub fn new() -> Self {
        let id_string = uuid::Uuid::new_v4().to_hyphenated().to_string();
        Self { id_string }
    }
}

impl From<String> for ObjectID {
    fn from(id_string: String) -> Self {
        Self { id_string }
    }
}

impl ToString for ObjectID {
    fn to_string(&self) -> String {
        self.id_string.clone()
    }
}
