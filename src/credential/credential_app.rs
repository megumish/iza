use crate::credential::*;
use futures::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::pin::Pin;

pub trait CredentialApp: HasCredentialDistributeService + HasCredentialRepository + Sync {
    fn init(&'static self, working_directory: &'static str) -> RetFuture<()> {
        future::try_join(
            self.credential_repository().init(working_directory),
            self.credential_distribute_service()
                .init(CredentialKind::SSHConnection, working_directory),
        )
        .and_then(|_| future::ready(Ok(())))
        .boxed()
    }

    fn new_credential(
        &'static self,
        kind: String,
        info: HashMap<String, String>,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let working_directory2 = working_directory.clone();
        future::ready(kind.try_into())
            .and_then(move |k| {
                self.credential_distribute_service().new_credential_of_kind(
                    k,
                    info,
                    working_directory,
                )
            })
            .and_then(move |c| future::ready(Ok((c.get_credential_id(), c.get_credential_kind()))))
            .and_then(move |(id, kind)| future::ready(Credential::restore(id, kind)))
            .and_then(move |c| self.credential_repository().push(&c, &working_directory2))
            .boxed()
    }

    fn credentials(
        &'static self,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Credential>>> + Send>> {
        self.credential_repository()
            .credentials(&working_directory)
            .boxed()
    }

    fn credentials_of_kind(
        &'static self,
        kind: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Credential>>> + Send>> {
        unimplemented!()
    }

    fn deploy_object(
        &'static self,
        id: String,
        local_path: String,
        remote_path: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        self.credential_repository()
            .credential_of_id(&id.into(), &working_directory)
            .and_then(move |c| {
                self.credential_distribute_service().deploy_object(
                    c.kind_of_credential(),
                    c.id_of_credential(),
                    local_path,
                    remote_path,
                    working_directory,
                )
            })
            .boxed()
    }
}

pub trait HasCredentialApp {
    type App: CredentialApp;

    fn credential_app(&self) -> &Self::App;
}

impl<T> CredentialApp for T where T: HasCredentialDistributeService + HasCredentialRepository + Sync {}
