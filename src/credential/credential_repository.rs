use crate::credential::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait CredentialRepository {
    fn push(&self, credential: Credential) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
}

pub trait HasCredentialRepository {
    type Repository: CredentialRepository;

    fn credential_repository(&self) -> &Self::Repository;
}
