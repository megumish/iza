use crate::ssh_connection::*;

pub struct RemoteFile {
    user_name: UserName,
    host_name: HostName,
    local_path: LocalPath,
    remote_path: RemotePath,
}

impl RemoteFile {
    pub fn restore(
        user_name: UserName,
        host_name: HostName,
        local_path: LocalPath,
        remote_path: RemotePath,
    ) -> Self {
        Self {
            user_name,
            host_name,
            local_path,
            remote_path,
        }
    }

    pub fn user_name_of_remote_file(&self) -> UserName {
        self.user_name.clone()
    }

    pub fn host_name_of_remote_file(&self) -> HostName {
        self.host_name.clone()
    }

    pub fn local_path_of_remote_file(&self) -> LocalPath {
        self.local_path.clone()
    }

    pub fn remote_path_of_remote_file(&self) -> RemotePath {
        self.remote_path.clone()
    }
}
