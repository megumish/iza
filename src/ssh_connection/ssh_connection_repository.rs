use crate::ssh_connection::*;

pub trait SSHConnectionRepository {
    fn push(&self, ssh_connection: &SSHConnection) -> Result<()>;

    fn remove(&self, ssh_connection: &SSHConnection) -> Result<()>;

    fn ssh_connections(&self, working_directory: &WorkingDirectory) -> Result<Vec<SSHConnection>>;

    fn ssh_connection_of_id(
        &self,
        ssh_connection_id: &SSHConnectionID,
        working_directory: &WorkingDirectory,
    ) -> Result<SSHConnection>;
}

pub trait HasSSHConnectionRepository {
    type Repository: SSHConnectionRepository;

    fn ssh_connection_repository(&self) -> &Self::Repository;
}
