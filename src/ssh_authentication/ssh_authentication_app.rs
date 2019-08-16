use crate::ssh_authentication::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait SSHAuthenticationApp:
    HasSSHAuthenticationRepository + HasRemoteFileRepository + Sync
{
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

    fn scp(
        &'static self,
        ssh_authentication_id: String,
        local_path: String,
        remote_path: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>>>> {
        future::ready(
            self.ssh_authentication_repository()
                .ssh_authentication_of_id(&ssh_authentication_id.into(), &working_directory.into()),
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

pub trait HasSSHAuthenticationApp {
    type App: SSHAuthenticationApp;

    fn ssh_authentication_app(&self) -> &Self::App;
}

impl<T> SSHAuthenticationApp for T where
    T: HasSSHAuthenticationRepository + HasRemoteFileRepository + Sync
{
}
