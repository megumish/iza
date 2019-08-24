use crate::dot_iza::*;
use crate::ssh_connection::*;
use std::sync::Arc;

#[derive(PartialEq, PartialOrd, Clone, Hash, Ord, Eq, Serialize, Deserialize)]
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

impl YamlModule<SSHConnection> for YamlSSHConnection {
    fn new_yaml_module(ssh_connection: Arc<SSHConnection>) -> Self {
        let ssh_connection: SSHConnection = (&*ssh_connection).clone();
        let id = ssh_connection.id_of_ssh_connection().to_string();
        let user = ssh_connection.user_of_ssh_connection().to_string();
        let host = ssh_connection.host_of_ssh_connection().to_string();
        Self { id, user, host }
    }

    fn restore(&self) -> SSHConnection {
        SSHConnection::restore(self.id.clone(), self.user.clone(), self.host.clone())
    }
}
