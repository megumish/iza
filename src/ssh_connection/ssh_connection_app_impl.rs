use crate::ssh_connection::*;

pub struct SSHConnectionAppImpl;

impl HasSSHConnectionRepository for SSHConnectionAppImpl {
    type Repository = SSHConnectionRepositoryDefaultImpl;

    fn ssh_connection_repository(&self) -> &Self::Repository {
        &SSHConnectionRepositoryDefaultImpl
    }
}

impl HasRemoteFileRepository for SSHConnectionAppImpl {
    type Repository = RemoteFileRepositoryDefaultImpl;

    fn remote_file_repository(&self) -> &Self::Repository {
        &RemoteFileRepositoryDefaultImpl
    }
}
