#[derive(Clone)]
pub struct UserName {
    name_string: String,
}

impl UserName {
    pub fn new(name_string: String) -> Self {
        Self { name_string }
    }
}

impl ToString for UserName {
    fn to_string(&self) -> String {
        self.name_string.clone()
    }
}
