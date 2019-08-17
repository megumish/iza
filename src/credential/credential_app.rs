use crate::credential::*;
use futures::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::pin::Pin;

pub trait CredentialApp: HasCredentialDistributeService + HasCredentialRepository + Sync {
    fn new_credential(
        &'static self,
        kind: String,
        info: HashMap<String, String>,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
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
            .and_then(move |c| self.credential_repository().push(c))
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
}
