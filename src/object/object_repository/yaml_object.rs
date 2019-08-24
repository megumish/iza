use crate::dot_iza::*;
use crate::object::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct YamlObject {
    id: String,
    package_id: String,
    object_info_id: String,
}

impl YamlModule<Object> for YamlObject {
    fn new_yaml_module(object: Arc<Object>) -> Self {
        let object: Object = (&*object).clone();
        let id = object.id_of_object().to_string();
        let package_id = object.package_id_of_object().to_string();
        let object_info_id = object.object_info_id_of_object().to_string();
        Self {
            id,
            package_id,
            object_info_id,
        }
    }

    fn restore(&self) -> Object {
        Object::restore(
            self.id.clone(),
            self.object_info_id.clone(),
            self.package_id.clone(),
        )
    }
}
