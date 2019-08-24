#[derive(Clone)]
pub struct Host {
    name_string: String,
}

impl Host {
    pub fn new(name_string: String) -> Self {
        Self { name_string }
    }
}

impl ToString for Host {
    fn to_string(&self) -> String {
        self.name_string.clone()
    }
}
