use crate::ssh_connection::*;

pub struct SSHConnection {
    id: SSHConnectionID,
    user_name: UserName,
    host_name: HostName,
    working_directory: WorkingDirectory,
}

impl SSHConnection {
    pub fn new(user_name: String, host_name: String, working_directory: String) -> Self {
        let id = SSHConnectionID::new();
        let user_name = UserName::new(user_name);
        let host_name = HostName::new(host_name);
        let working_directory = WorkingDirectory::new(working_directory);
        Self {
            id,
            user_name,
            host_name,
            working_directory,
        }
    }

    pub fn user_name(&self) -> UserName {
        self.user_name.clone()
    }

    pub fn host_name(&self) -> HostName {
        self.host_name.clone()
    }

    pub fn working_directory(&self) -> WorkingDirectory {
        self.working_directory.clone()
    }
}
