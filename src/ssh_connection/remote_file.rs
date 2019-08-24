use crate::ssh_connection::*;

pub struct RemoteFile {
    user: User,
    host: Host,
    local_path: LocalPath,
    remote_path: RemotePath,
}

impl RemoteFile {
    pub fn restore(user: User, host: Host, local_path: LocalPath, remote_path: RemotePath) -> Self {
        Self {
            user,
            host,
            local_path,
            remote_path,
        }
    }

    pub fn user_of_remote_file(&self) -> User {
        self.user.clone()
    }

    pub fn host_of_remote_file(&self) -> Host {
        self.host.clone()
    }

    pub fn local_path_of_remote_file(&self) -> LocalPath {
        self.local_path.clone()
    }

    pub fn remote_path_of_remote_file(&self) -> RemotePath {
        self.remote_path.clone()
    }
}
