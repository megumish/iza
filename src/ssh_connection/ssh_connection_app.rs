use crate::ssh_connection::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait SSHConnectionApp: HasSSHConnectionRepository + HasRemoteFileRepository + Sync {
    fn new_ssh_connection(
        &'static self,
        user_name: String,
        host_name: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        future::ready(Ok(SSHConnection::new(
            user_name,
            host_name,
            working_directory,
        )))
        .and_then(move |s| future::ready(self.ssh_connection_repository().push(&s)))
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
    ) -> Pin<Box<dyn Future<Output = Result<SSHConnection>>>> {
        future::ready(
            self.ssh_connection_repository()
                .ssh_connection_of_id(&ssh_connection_id.into(), &working_directory.into()),
        )
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
    ) -> Pin<Box<dyn Future<Output = Result<()>>>> {
        future::ready(
            self.ssh_connection_repository()
                .ssh_connection_of_id(&ssh_connection_id.into(), &working_directory.into()),
        )
        .and_then(move |s| {
            let local_path: LocalPath = local_path.into();
            let remote_path: RemotePath = remote_path.into();
            let user_name: UserName = s.user_name();
            let host_name: HostName = s.host_name();
            let working_directory: WorkingDirectory = s.working_directory();
            future::ready(Ok(RemoteFile::restore(
                user_name,
                host_name,
                local_path,
                remote_path,
                working_directory,
            )))
        })
        .and_then(move |r| future::ready(self.remote_file_repository().push(&r)))
        .boxed()
    }
}

pub trait HasSSHConnectionApp {
    type App: SSHConnectionApp;

    fn ssh_connection_app(&self) -> &Self::App;
}

impl<T> SSHConnectionApp for T where T: HasSSHConnectionRepository + HasRemoteFileRepository + Sync {}
