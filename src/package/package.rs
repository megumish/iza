use crate::package::*;

pub struct Package {
    name: PackageName,
}

impl Package {
    pub fn new(name: String) -> Self {
        let name = PackageName::new(name);
        Self { name }
    }

    pub fn name_of_package(&self) -> String {
        self.name.to_string()
    }
}
