mod yaml_ssh_connection;

use self::yaml_ssh_connection::*;
use crate::dot_iza::*;
use crate::ssh_connection::*;
use futures::prelude::*;
use std::sync::Arc;

use crate::ssh_connection::{ErrorKind, ResultFuture};

pub trait SSHConnectionRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()>;

    fn push(
        &self,
        ssh_connection: Arc<SSHConnection>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<SSHConnection>>;

    fn delete(
        &self,
        ssh_connection: Arc<SSHConnection>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<SSHConnection>>;

    fn ssh_connection_of_id(
        &self,
        package_id: Arc<SSHConnectionID>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<SSHConnection>>;
}

pub struct DotIzaSSHConnectionRepository;

const PRURAL_NAME: &'static str = "ssh_connections";

impl SSHConnectionRepository for DotIzaSSHConnectionRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()> {
        init_module_file(working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn push(
        &self,
        ssh_connection: Arc<SSHConnection>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<SSHConnection>> {
        push_module::<_, YamlSSHConnection>(ssh_connection, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn delete(
        &self,
        ssh_connection: Arc<SSHConnection>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<SSHConnection>> {
        delete_module::<_, YamlSSHConnection>(ssh_connection, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn ssh_connection_of_id(
        &self,
        id: Arc<SSHConnectionID>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<SSHConnection>> {
        let id2 = id.clone();
        modules_under_condition::<_, YamlSSHConnection, _>(
            move |s| &s.id_of_ssh_connection() == &*id,
            working_directory,
            PRURAL_NAME,
        )
        .map_err(Into::into)
        .and_then(move |ss| {
            future::lazy(move |_| {
                if let Some(s) = ss.first() {
                    Ok(s.clone())
                } else {
                    let id: SSHConnectionID = (&*id2).clone();
                    Err(ErrorKind::NotFoundSSHConnection(id).into())
                }
            })
        })
        .boxed()
    }
}

pub trait HasSSHConnectionRepository {
    type Repository: SSHConnectionRepository;

    fn ssh_connection_repository(&self) -> &Self::Repository;
}
