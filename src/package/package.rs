use crate::dot_iza::*;
use crate::package::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    name: PackageName,
}

impl Package {
    pub fn new(name: String) -> Self {
        let name = PackageName::new(name);
        Self { name }
    }

    pub fn new_arc(name: String) -> Arc<Self> {
        Arc::new(Self::new(name))
    }

    pub fn name_of_package(&self) -> PackageName {
        self.name.clone()
    }
}

impl Module for Package {}
