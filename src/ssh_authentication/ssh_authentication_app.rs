use crate::ssh_authentication::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait SSHAuthenticationApp: HasSSHAuthenticationRepository + Sync {
    fn new_ssh_authentication(
        &'static self,
        user_name: String,
        host_name: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>>>> {
        future::ready(Ok(SSHAuthentication::new(
            user_name,
            host_name,
            working_directory,
        )))
        .and_then(move |s| future::ready(self.ssh_authentication_repository().push(&s)))
        .boxed()
    }

    fn ssh_authentications(
        &'static self,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SSHAuthentication>>>>> {
        future::ready(
            self.ssh_authentication_repository()
                .ssh_authentications(&working_directory.into()),
        )
        .boxed()
    }

    fn ssh_authentication_of_id(
        &'static self,
        ssh_authentication_id: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<SSHAuthentication>>>> {
        future::ready(
            self.ssh_authentication_repository()
                .ssh_authentication_of_id(&ssh_authentication_id.into(), &working_directory.into()),
        )
        .boxed()
    }

    fn remove_ssh_authentication_of_id(
        &'static self,
        ssh_authentication_id: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>>>> {
        future::ready(
            self.ssh_authentication_repository()
                .ssh_authentication_of_id(&ssh_authentication_id.into(), &working_directory.into()),
        )
        .and_then(move |s| future::ready(self.ssh_authentication_repository().remove(&s)))
        .boxed()
    }
}

pub trait HasSSHAuthenticationApp {
    type App: SSHAuthenticationApp;

    fn ssh_authentication_app(&self) -> &Self::App;
}

impl<T> SSHAuthenticationApp for T where T: HasSSHAuthenticationRepository + Sync {}
