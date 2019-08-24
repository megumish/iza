mod yaml_credential;

use self::yaml_credential::*;
use crate::credential::*;
use crate::dot_iza::*;
use futures::prelude::*;
use std::sync::Arc;

use crate::credential::{ErrorKind, ResultFuture};

pub trait CredentialRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()>;

    fn push(
        &self,
        credential: Arc<Credential>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Credential>>;

    fn delete(
        &self,
        credential: Arc<Credential>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Credential>>;

    fn credentials(&self, working_directory: &'static str) -> ResultFuture<Vec<Arc<Credential>>>;

    fn credential_of_id(
        &self,
        id: Arc<CredentialID>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Credential>>;
}

pub struct DotIzaCredentialRepository;

const PRURAL_NAME: &'static str = "credentials";

impl CredentialRepository for DotIzaCredentialRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()> {
        init_module_file(working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn push(
        &self,
        credential: Arc<Credential>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Credential>> {
        push_module::<_, YamlCredential>(credential, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn delete(
        &self,
        credential: Arc<Credential>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Credential>> {
        delete_module::<_, YamlCredential>(credential, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn credentials(&self, working_directory: &'static str) -> ResultFuture<Vec<Arc<Credential>>> {
        modules_under_condition::<_, YamlCredential, _>(
            move |_| true,
            working_directory,
            PRURAL_NAME,
        )
        .map_err(Into::into)
        .boxed()
    }

    fn credential_of_id(
        &self,
        id: Arc<CredentialID>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Credential>> {
        let id2 = id.clone();
        modules_under_condition::<_, YamlCredential, _>(
            move |c| &c.id_of_credential() == &*id,
            working_directory,
            PRURAL_NAME,
        )
        .map_err(Into::into)
        .and_then(move |cs| {
            future::lazy(move |_| {
                if let Some(c) = cs.first() {
                    Ok(c.clone())
                } else {
                    let id: CredentialID = (&*id2).clone();
                    Err(ErrorKind::NotFoundCredential(id).into())
                }
            })
        })
        .boxed()
    }
}

pub trait HasCredentialRepository {
    type Repository: CredentialRepository;

    fn credential_repository(&self) -> &Self::Repository;
}
