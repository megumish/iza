use crate::dot_iza::*;
use crate::object::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Object {
    id: ObjectID,
    package_id: PackageID,
    object_info_id: ObjectInfoID,
}

impl Object {
    fn new(object_info_id: ObjectInfoID, package_id: String) -> Self {
        let id = ObjectID::new();
        let package_id: PackageID = package_id.into();

        Self {
            id,
            package_id,
            object_info_id,
        }
    }

    pub fn new_arc(object_info_id: ObjectInfoID, package_id: String) -> Arc<Self> {
        Arc::new(Self::new(object_info_id, package_id))
    }

    pub fn restore(id: String, object_info_id: String, package_id: String) -> Self {
        let id: ObjectID = id.into();
        let object_info_id: ObjectInfoID = object_info_id.into();
        let package_id: PackageID = package_id.into();

        Self {
            id,
            package_id,
            object_info_id,
        }
    }

    pub fn id_of_object(&self) -> ObjectID {
        self.id.clone()
    }

    pub fn package_id_of_object(&self) -> PackageID {
        self.package_id.clone()
    }

    pub fn object_info_id_of_object(&self) -> ObjectInfoID {
        self.object_info_id.clone()
    }
}

impl Module for Object {}
