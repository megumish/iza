#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct YamlObjectInfo {
    id: String,
    local_path: String,
    remote_path: String,
    credential_id: String,
}

impl YamlObjectInfo {
    pub fn new(id: String, local_path: String, remote_path: String, credential_id: String) -> Self {
        Self {
            id,
            local_path,
            remote_path,
            credential_id,
        }
    }

    pub fn id_of_yaml_object_info(&self) -> String {
        self.id.to_string()
    }

    pub fn local_path_of_yaml_object_info(&self) -> String {
        self.local_path.to_string()
    }

    pub fn remote_path_of_yaml_object_info(&self) -> String {
        self.remote_path.to_string()
    }

    pub fn credential_id_of_yaml_object_info(&self) -> String {
        self.credential_id.to_string()
    }
}
