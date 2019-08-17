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
