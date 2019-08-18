mod yaml_ssh_connection;

use self::yaml_ssh_connection::*;
use crate::ssh_connection::*;
use futures::prelude::*;
use serde_yaml as yaml;
use std::fs;
use std::io::prelude::*;
use std::path;
use std::pin::Pin;

pub trait SSHConnectionRepository {
    fn push(
        &self,
        ssh_connection: &SSHConnection,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

    fn remove(&self, ssh_connection: &SSHConnection) -> Result<()>;

    fn ssh_connections(&self, working_directory: &WorkingDirectory) -> Result<Vec<SSHConnection>>;

    fn ssh_connection_of_id(
        &self,
        ssh_connection_id: &SSHConnectionID,
        working_directory: &WorkingDirectory,
    ) -> Result<SSHConnection>;
}

pub struct SSHConnectionRepositoryDefaultImpl;

impl SSHConnectionRepository for SSHConnectionRepositoryDefaultImpl {
    fn push(
        &self,
        ssh_connection: &SSHConnection,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let ssh_connection = ssh_connection.clone();
        let working_directory = working_directory.to_owned();
        future::lazy(move |_| {
            let id = ssh_connection.id_of_ssh_connection();
            let user = ssh_connection.user_name_of_ssh_connection();
            let host = ssh_connection.host_name_of_ssh_connection();

            let ssh_connections_path_buf = {
                let mut p = path::Path::new(&working_directory).to_path_buf();
                p.push(".iza");
                p.push("credential");
                p.push("ssh_connection");
                p
            };

            let new_ssh_connections = {
                let mut input_data = Vec::new();
                let mut ssh_connections_file = fs::File::open(&ssh_connections_path_buf)?;
                ssh_connections_file.read_to_end(&mut input_data)?;
                let mut ssh_connections: Vec<YamlSSHConnection> = if input_data.is_empty() {
                    Vec::new()
                } else {
                    yaml::from_slice(&input_data)?
                };
                match ssh_connections
                    .iter()
                    .find(|p| p.id_of_yaml_ssh_connection() == id.to_string())
                {
                    Some(_) => return Err(Error::AlreadyExistSSHConnection),
                    None => { /* do nothing */ }
                }
                ssh_connections.push(YamlSSHConnection::new(
                    id.to_string(),
                    user.to_string(),
                    host.to_string(),
                ));
                ssh_connections
            };

            {
                let output_data = yaml::to_vec(&new_ssh_connections)?;
                let mut ssh_connections_file = fs::File::create(&ssh_connections_path_buf)?;
                ssh_connections_file.write(&output_data)?;
            }

            Ok(())
        })
        .boxed()
    }

    fn remove(&self, ssh_connection: &SSHConnection) -> Result<()> {
        unimplemented!()
    }

    fn ssh_connections(&self, working_directory: &WorkingDirectory) -> Result<Vec<SSHConnection>> {
        let working_directory = working_directory.to_string();

        let ssh_connections_path_buf = {
            let mut p = path::Path::new(&working_directory).to_path_buf();
            p.push(".iza");
            p.push("credential");
            p.push("ssh_connection");
            p
        };

        Ok({
            let mut input_data = Vec::new();
            let mut ssh_connections_file = fs::File::open(&ssh_connections_path_buf)?;
            ssh_connections_file.read_to_end(&mut input_data)?;
            let ssh_connections: Vec<YamlSSHConnection> = if input_data.is_empty() {
                Vec::new()
            } else {
                yaml::from_slice(&input_data)?
            };
            ssh_connections
                .iter()
                .map(|p| {
                    SSHConnection::new(
                        p.id_of_yaml_ssh_connection(),
                        p.user_of_yaml_ssh_connection(),
                        p.host_of_yaml_ssh_connection(),
                    )
                })
                .collect()
        })
    }

    fn ssh_connection_of_id(
        &self,
        ssh_connection_id: &SSHConnectionID,
        working_directory: &WorkingDirectory,
    ) -> Result<SSHConnection> {
        unimplemented!()
    }
}

pub trait HasSSHConnectionRepository {
    type Repository: SSHConnectionRepository;

    fn ssh_connection_repository(&self) -> &Self::Repository;
}
