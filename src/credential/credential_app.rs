use crate::credential::*;
use futures::prelude::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::pin::Pin;

pub trait CredentialApp: HasCredentialDistributeService + Sync {
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
            .boxed()
    }
}
