use crate::credential::*;
use crate::ssh_connection::*;
use futures::prelude::*;
use std::pin::Pin;

use crate::ssh_connection::{Error, Result, RetFuture};

pub trait SSHConnectionApp: HasSSHConnectionRepository + HasRemoteFileRepository + Sync {
    fn init(&'static self, working_directory: &'static str) -> RetFuture<()> {
        self.ssh_connection_repository()
            .init(working_directory)
            .boxed()
    }
    fn new_ssh_connection(
        &'static self,
        user_name: String,
        host_name: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<Box<dyn CredentialAs + Send>>> + Send>> {
        let working_directory2 = working_directory.clone();
        future::ready(Ok(SSHConnection::new(
            user_name,
            host_name,
            working_directory,
        )))
        .and_then(move |s| {
            future::try_join(
                self.ssh_connection_repository()
                    .push(&s, &working_directory2),
                {
                    let s: Box<dyn CredentialAs + Send> = Box::new(s);
                    future::ready(Ok(s))
                },
            )
            .or_else(|e| future::ready(Err(e)))
            .and_then(|((), s)| future::ready(Ok(s)))
        })
        .boxed()
    }

    fn ssh_connections(
        &'static self,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SSHConnection>>>>> {
        future::ready(
            self.ssh_connection_repository()
                .ssh_connections(&working_directory.into()),
        )
        .boxed()
    }

    fn ssh_connection_of_id(
        &'static self,
        ssh_connection_id: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<Box<dyn CredentialAs + Send>>> + Send>> {
        future::ready(
            self.ssh_connection_repository()
                .ssh_connection_of_id(&ssh_connection_id.into(), &working_directory.into()),
        )
        .and_then(|s| {
            let s: Box<dyn CredentialAs + Send> = Box::new(s);
            future::ready(Ok(s))
        })
        .boxed()
    }

    fn remove_ssh_connection_of_id(
        &'static self,
        ssh_connection_id: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>>>> {
        future::ready(
            self.ssh_connection_repository()
                .ssh_connection_of_id(&ssh_connection_id.into(), &working_directory.into()),
        )
        .and_then(move |s| future::ready(self.ssh_connection_repository().remove(&s)))
        .boxed()
    }

    fn scp(
        &'static self,
        ssh_connection_id: String,
        local_path: String,
        remote_path: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let working_directory2 = working_directory.clone();
        future::ready(
            self.ssh_connection_repository()
                .ssh_connection_of_id(&ssh_connection_id.into(), &working_directory.into()),
        )
        .and_then(move |s| {
            let local_path: LocalPath = local_path.into();
            let remote_path: RemotePath = remote_path.into();
            let user_name: UserName = s.user_name_of_ssh_connection();
            let host_name: HostName = s.host_name_of_ssh_connection();
            future::ready(Ok(RemoteFile::restore(
                user_name,
                host_name,
                local_path,
                remote_path,
            )))
        })
        .and_then(move |r| {
            future::ready(self.remote_file_repository().push(&r, &working_directory2))
        })
        .boxed()
    }
}

pub trait HasSSHConnectionApp {
    type App: SSHConnectionApp;

    fn ssh_connection_app(&self) -> &Self::App;
}

impl<T> SSHConnectionApp for T where T: HasSSHConnectionRepository + HasRemoteFileRepository + Sync {}
