use crate::credential::*;
use crate::ssh_connection::*;
use futures::prelude::*;
use std::collections::HashMap;
use std::pin::Pin;

use crate::credential::{Error, Result};

pub trait CredentialDistributeService: HasSSHConnectionApp + Sync {
    fn new_credential_of_kind(
        &'static self,
        kind: CredentialKind,
        info: HashMap<String, String>,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        use CredentialKind::*;
        match kind {
            SSHConnection => {
                let mut not_enough_info = Vec::new();

                let user_name;
                match info.get("user") {
                    Some(n) => user_name = Some(n),
                    None => {
                        user_name = None;
                        not_enough_info.push("user".to_owned());
                    }
                }
                let host_name;
                match info.get("host") {
                    Some(h) => host_name = Some(h),
                    None => {
                        host_name = None;
                        not_enough_info.push("host".to_owned());
                    }
                }

                if !not_enough_info.is_empty() {
                    return future::ready(Err(Error::NotEnoughInfo(not_enough_info))).boxed();
                }
                self.ssh_connection_app()
                    .new_ssh_connection(
                        user_name.expect("UserName must be existed").to_owned(),
                        host_name.expect("HostName must be existed").to_owned(),
                        working_directory,
                    )
                    .map_err(|e| {
                        eprintln!("{}", e);
                        Error::OtherAppError
                    })
                    .boxed()
            }
        }
    }
}

pub trait HasCredentialDistributeService {
    type Service: CredentialDistributeService;

    fn credential_distribute_service(&self) -> &Self::Service;
}
