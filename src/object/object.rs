use crate::object::*;

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
}
