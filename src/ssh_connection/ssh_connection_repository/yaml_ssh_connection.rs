use crate::dot_iza::*;
use crate::ssh_connection::*;
use std::sync::Arc;

#[derive(PartialEq, PartialOrd, Clone, Hash, Ord, Eq, Serialize, Deserialize)]
pub struct YamlSSHConnection {
    id: String,
    user: String,
    host: String,
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
