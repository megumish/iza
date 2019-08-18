use crate::credential::*;
use crate::ssh_connection::*;

pub struct CredentialAppImpl;

impl HasCredentialRepository for CredentialAppImpl {
    type Repository = CredentialRepositoryDefaultImpl;

    fn credential_repository(&self) -> &Self::Repository {
        &CredentialRepositoryDefaultImpl
    }
}

impl HasCredentialDistributeService for CredentialAppImpl {
    type Service = Self;

    fn credential_distribute_service(&self) -> &Self::Service {
        self
    }
}

impl HasSSHConnectionApp for CredentialAppImpl {
    type App = SSHConnectionAppImpl;

    fn ssh_connection_app(&self) -> &Self::App {
        &SSHConnectionAppImpl
    }
}
