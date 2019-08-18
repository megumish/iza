#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub struct YamlSSHConnection {
    id: String,
    user: String,
    host: String,
}

impl YamlSSHConnection {
    pub fn new(id: String, user: String, host: String) -> Self {
        Self { id, user, host }
    }

    pub fn id_of_yaml_ssh_connection(&self) -> String {
        self.id.to_string()
    }

    pub fn user_of_yaml_ssh_connection(&self) -> String {
        self.user.to_string()
    }

    pub fn host_of_yaml_ssh_connection(&self) -> String {
        self.host.to_string()
    }
}
