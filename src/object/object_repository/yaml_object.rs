#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct YamlObject {
    id: String,
    package_id: String,
    object_info_id: String,
}

impl YamlObject {
    pub fn new(id: String, package_id: String, object_info_id: String) -> Self {
        Self {
            id,
            package_id,
            object_info_id,
        }
    }

    pub fn id_of_yaml_object(&self) -> String {
        self.id.to_string()
    }

    pub fn package_id_of_yaml_object(&self) -> String {
        self.package_id.to_string()
    }

    pub fn object_info_id_of_yaml_object(&self) -> String {
        self.object_info_id.to_string()
    }
}
