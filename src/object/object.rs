use crate::object::*;

#[derive(Clone)]
pub struct Object {
    id: ObjectID,
    package_id: PackageID,
    object_info_id: ObjectInfoID,
}

impl Object {
    pub fn new(object_info_id: ObjectInfoID, package_id: String) -> Self {
        let id = ObjectID::new();
        let package_id: PackageID = package_id.into();

        Self {
            id,
            package_id,
            object_info_id,
        }
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

    pub fn id_of_object(&self) -> String {
        self.id.to_string()
    }

    pub fn package_id_of_object(&self) -> String {
        self.package_id.to_string()
    }

    pub fn object_info_id_of_object(&self) -> String {
        self.object_info_id.to_string()
    }
}
