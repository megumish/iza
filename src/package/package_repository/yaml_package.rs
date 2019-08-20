use crate::dot_iza::*;
use crate::package::*;
use std::sync::Arc;

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Serialize, Deserialize)]
pub struct YamlPackage {
    name: String,
}

impl YamlPackage {}

impl YamlModule<Package> for YamlPackage {
    fn new_yaml_module(package: Arc<Package>) -> Self {
        let name: PackageName = (&*package).into();
        let name = name.to_string();
        Self { name }
    }
}
