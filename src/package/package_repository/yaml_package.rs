use crate::dot_iza::*;
use crate::package::*;
use std::sync::Arc;

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Serialize, Deserialize)]
pub struct YamlPackage {
    name: String,
}

impl YamlModule<Package> for YamlPackage {
    fn new_yaml_module(package: Arc<Package>) -> Self {
        let name: PackageName = (&*package).name_of_package();
        let name = name.to_string();
        Self { name }
    }

    fn restore(&self) -> Package {
        Package::new(self.name.clone())
    }
}
