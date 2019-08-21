use crate::dot_iza::*;
use crate::object::*;
use std::sync::Arc;

#[derive(PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct YamlObjectInfo {
    id: String,
    local_path: String,
    remote_path: String,
    credential_id: String,
}

impl YamlModule<ObjectInfo> for YamlObjectInfo {
    fn new_yaml_module(object_info: Arc<ObjectInfo>) -> Self {
        let object_info: ObjectInfo = (&*object_info).clone();
        let id = object_info.id_of_object_info().to_string();
        let local_path = object_info.local_path_of_object_info().to_string();
        let remote_path = object_info.remote_path_of_object_info().to_string();
        let credential_id = object_info.credential_id_of_object_info().to_string();
        Self {
            id,
            local_path,
            remote_path,
            credential_id,
        }
    }

    fn restore(&self) -> ObjectInfo {
        ObjectInfo::restore(
            self.id.clone(),
            self.local_path.clone(),
            self.remote_path.clone(),
            self.credential_id.clone(),
        )
    }
}
