use crate::package::*;

pub struct Package {
    name: PackageName,
    working_directory: WorkingDirectory,
}

impl Package {
    pub fn new(name: String, working_directory: String) -> Self {
        let name = PackageName::new(name);
        let working_directory = WorkingDirectory::new(working_directory);
        Self {
            name,
            working_directory,
        }
    }

    pub fn name_of_package(&self) -> String {
        self.name.to_string()
    }

    pub fn working_directory_of_package(&self) -> String {
        self.working_directory.to_string()
    }
}
