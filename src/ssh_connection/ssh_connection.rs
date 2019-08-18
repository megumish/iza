use crate::credential::*;
use crate::ssh_connection::*;

#[derive(Clone)]
pub struct SSHConnection {
    id: SSHConnectionID,
    user_name: UserName,
    host_name: HostName,
}

impl SSHConnection {
    pub fn new(user_name: String, host_name: String, working_directory: String) -> Self {
        let id = SSHConnectionID::new();
        let user_name = UserName::new(user_name);
        let host_name = HostName::new(host_name);
        Self {
            id,
            user_name,
            host_name,
        }
    }

    pub fn id_of_ssh_connection(&self) -> SSHConnectionID {
        self.id.clone()
    }

    pub fn user_name_of_ssh_connection(&self) -> UserName {
        self.user_name.clone()
    }

    pub fn host_name_of_ssh_connection(&self) -> HostName {
        self.host_name.clone()
    }
}

impl CredentialAs for SSHConnection {
    fn get_credential_id(&self) -> String {
        self.id.to_string()
    }

    fn get_credential_kind(&self) -> String {
        "SSHConnection".to_owned()
    }
}
