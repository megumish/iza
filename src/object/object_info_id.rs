#[derive(Clone)]
pub struct ObjectInfoID {
    id_string: String,
}

impl ObjectInfoID {
    pub fn new() -> Self {
        let id_string = uuid::Uuid::new_v4().to_hyphenated().to_string();
        Self { id_string }
    }
}

impl From<String> for ObjectInfoID {
    fn from(id_string: String) -> Self {
        Self { id_string }
    }
}

impl ToString for ObjectInfoID {
    fn to_string(&self) -> String {
        self.id_string.clone()
    }
}
