use crate::credential::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait CredentialRepository {
    fn push(&self, credential: Credential) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

    fn credentials(
        &self,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Credential>>> + Send>>;

    fn credential_of_id(
        &self,
        id: &CredentialID,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Credential>> + Send>>;
}

pub trait HasCredentialRepository {
    type Repository: CredentialRepository;

    fn credential_repository(&self) -> &Self::Repository;
}
