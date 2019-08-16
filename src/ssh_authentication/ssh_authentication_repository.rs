use crate::ssh_authentication::*;

pub trait SSHAuthenticationRepository {
    fn push(&self, ssh_authentication: &SSHAuthentication) -> Result<()>;

    fn remove(&self, ssh_authentication: &SSHAuthentication) -> Result<()>;

    fn ssh_authentications(
        &self,
        working_directory: &WorkingDirectory,
    ) -> Result<Vec<SSHAuthentication>>;

    fn ssh_authentication_of_id(
        &self,
        ssh_authentication_id: &SSHAuthenticationID,
        working_directory: &WorkingDirectory,
    ) -> Result<SSHAuthentication>;
}

pub trait HasSSHAuthenticationRepository {
    type Repository: SSHAuthenticationRepository;

    fn ssh_authentication_repository(&self) -> &Self::Repository;
}
