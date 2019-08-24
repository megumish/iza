use crate::credential::*;
use crate::ssh_connection::*;
use futures::prelude::*;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use crate::credential::{ErrorKind, Result, ResultFuture};

pub trait CredentialDistributeService: HasSSHConnectionApp + Sync {
    fn init(
        &'static self,
        kind: CredentialKind,
        working_directory: &'static str,
    ) -> ResultFuture<()> {
        use CredentialKind::*;
        match kind {
            SSHConnection => SSHConnectionApp::init(self.ssh_connection_app(), working_directory)
                .map_err(Into::into)
                .boxed(),
        }
    }

    fn new_credential_of_kind(
        &'static self,
        kind: CredentialKind,
        info: Arc<HashMap<String, String>>,
        id: String,
        working_directory: &'static str,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<HashMap<String, String>>>> + Send>> {
        use CredentialKind::*;
        match kind {
            SSHConnection => self
                .ssh_connection_app()
                .new_ssh_connection(info, id, working_directory)
                .map_ok(|s| s.arc_hash_map())
                .map_err(Into::into)
                .boxed(),
        }
    }

    // fn deploy_object(
    //     &'static self,
    //     kind: CredentialKind,
    //     id: CredentialID,
    //     local_path: String,
    //     remote_path: String,
    //     working_directory: String,
    // ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
    //     use CredentialKind::*;
    //     match kind {
    //         SSHConnection => self
    //             .ssh_connection_app()
    //             .scp(id.to_string(), local_path, remote_path, working_directory)
    //             .map_err(|e| {
    //                 eprintln!("{}", e);
    //                 Error::OtherAppError
    //             })
    //             .boxed(),
    //     }
    // }
}

pub trait HasCredentialDistributeService {
    type Service: CredentialDistributeService;

    fn credential_distribute_service(&self) -> &Self::Service;
}

impl<T> CredentialDistributeService for T where T: HasSSHConnectionApp + Sync {}
