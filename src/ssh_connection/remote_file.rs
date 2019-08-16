use crate::ssh_connection::*;

pub struct RemoteFile {
    user_name: UserName,
    host_name: HostName,
    local_path: LocalPath,
    remote_path: RemotePath,
    working_directory: WorkingDirectory,
}

impl RemoteFile {
    pub fn restore(
        user_name: UserName,
        host_name: HostName,
        local_path: LocalPath,
        remote_path: RemotePath,
        working_directory: WorkingDirectory,
    ) -> Self {
        Self {
            user_name,
            host_name,
            local_path,
            remote_path,
            working_directory,
        }
    }
}
