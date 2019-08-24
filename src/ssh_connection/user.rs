#[derive(Clone)]
pub struct User {
    name_string: String,
}

impl User {
    pub fn new(name_string: String) -> Self {
        Self { name_string }
    }
}

impl ToString for User {
    fn to_string(&self) -> String {
        self.name_string.clone()
    }
}
