use crate::ssh_connection::*;

pub trait RemoteFileRepository {
    fn push(&self, remote_file: &RemoteFile) -> Result<()>;
}

pub trait HasRemoteFileRepository {
    type Repository: RemoteFileRepository;

    fn remote_file_repository(&self) -> &Self::Repository;
}
