use crate::credential::*;
use futures::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::pin::Pin;
use std::sync::Arc;

pub trait CredentialApp: HasCredentialRepository + HasCredentialDistributeService + Sync {
    fn init(&'static self, working_directory: &'static str) -> ResultFuture<()> {
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
        info: Arc<HashMap<String, String>>,
        kind: String,
        working_directory: &'static str,
    ) -> ResultFuture<(Arc<Credential>, Arc<HashMap<String, String>>)> {
        let info2 = info.clone();
        future::lazy(move |_| Credential::try_new_arc(info.clone(), kind))
            .and_then(move |c| {
                future::try_join(
                    self.credential_repository()
                        .push(c.clone(), working_directory),
                    self.credential_distribute_service().new_credential_of_kind(
                        c.kind_of_credential(),
                        info2,
                        c.id_of_credential().to_string(),
                        working_directory,
                    ),
                )
            })
            .boxed()
    }

    // fn credentials(
    //     &'static self,
    //     working_directory: String,
    // ) -> Pin<Box<dyn Future<Output = Result<Vec<Credential>>> + Send>> {
    //     self.credential_repository()
    //         .credentials(&working_directory)
    //         .boxed()
    // }

    // fn credentials_of_kind(
    //     &'static self,
    //     kind: String,
    //     working_directory: String,
    // ) -> Pin<Box<dyn Future<Output = Result<Vec<Credential>>> + Send>> {
    //     unimplemented!()
    // }

    // fn deploy_object(
    //     &'static self,
    //     id: String,
    //     local_path: String,
    //     remote_path: String,
    //     working_directory: String,
    // ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
    //     self.credential_repository()
    //         .credential_of_id(&id.into(), &working_directory)
    //         .and_then(move |c| {
    //             self.credential_distribute_service().deploy_object(
    //                 c.kind_of_credential(),
    //                 c.id_of_credential(),
    //                 local_path,
    //                 remote_path,
    //                 working_directory,
    //             )
    //         })
    //         .boxed()
    // }
}

pub trait HasCredentialApp {
    type App: CredentialApp;

    fn credential_app(&self) -> &Self::App;
}

impl<T> CredentialApp for T where T: HasCredentialRepository + HasCredentialDistributeService + Sync {}
