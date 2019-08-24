use crate::ssh_connection::*;

pub struct SSHConnectionAppImpl;

impl HasSSHConnectionRepository for SSHConnectionAppImpl {
    type Repository = DotIzaSSHConnectionRepository;

    fn ssh_connection_repository(&self) -> &Self::Repository {
        &DotIzaSSHConnectionRepository
    }
}

impl HasRemoteFileRepository for SSHConnectionAppImpl {
    type Repository = RemoteFileRepositoryDefaultImpl;

    fn remote_file_repository(&self) -> &Self::Repository {
        &RemoteFileRepositoryDefaultImpl
    }
}
