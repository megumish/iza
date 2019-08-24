use crate::credential::*;
use crate::ssh_connection::*;
use futures::prelude::*;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use crate::ssh_connection::{Error, ResultFuture};

pub trait SSHConnectionApp: HasSSHConnectionRepository + HasRemoteFileRepository + Sync {
    fn init(&'static self, working_directory: &'static str) -> ResultFuture<()> {
        self.ssh_connection_repository()
            .init(working_directory)
            .boxed()
    }

    fn new_ssh_connection(
        &'static self,
        info: Arc<HashMap<String, String>>,
        id: String,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<SSHConnection>> {
        future::lazy(|_| SSHConnection::try_arc_from_arc_hash_map(info, id))
            .map_err(Into::into)
            .and_then(move |s| {
                self.ssh_connection_repository()
                    .push(s.clone(), working_directory)
            })
            .boxed()
    }

    fn ssh_connection_of_id(
        &'static self,
        ssh_connection_id: String,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<SSHConnection>> {
        self.ssh_connection_repository()
            .ssh_connection_of_id(Arc::new(ssh_connection_id.into()), working_directory)
            .boxed()
    }

    // fn remove_ssh_connection_of_id(
    //     &'static self,
    //     ssh_connection_id: String,
    //     working_directory: String,
    // ) -> Pin<Box<dyn Future<Output = Result<()>>>> {
    //     future::ready(
    //         self.ssh_connection_repository()
    //             .ssh_connection_of_id(&ssh_connection_id.into(), &working_directory.into()),
    //     )
    //     .and_then(move |s| future::ready(self.ssh_connection_repository().remove(&s)))
    //     .boxed()
    // }

    fn scp(
        &'static self,
        ssh_connection_id: String,
        local_path: String,
        remote_path: String,
        working_directory: &'static str,
    ) -> ResultFuture<()> {
        self.ssh_connection_repository()
            .ssh_connection_of_id(Arc::new(ssh_connection_id.into()), working_directory.into())
            .and_then(move |s| {
                let local_path: LocalPath = local_path.into();
                let remote_path: RemotePath = remote_path.into();
                let user: User = s.user_of_ssh_connection();
                let host: Host = s.host_of_ssh_connection();
                future::lazy(|_| Ok(RemoteFile::restore(user, host, local_path, remote_path)))
            })
            .and_then(move |r| {
                future::ready(self.remote_file_repository().push(&r, working_directory))
            })
            .boxed()
    }
}

pub trait HasSSHConnectionApp {
    type App: SSHConnectionApp;

    fn ssh_connection_app(&self) -> &Self::App;
}

impl<T> SSHConnectionApp for T where T: HasSSHConnectionRepository + HasRemoteFileRepository + Sync {}
