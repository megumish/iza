mod yaml_credential;

use self::yaml_credential::*;
use crate::credential::*;
use futures::prelude::*;
use serde_yaml as yaml;
use std::fs;
use std::io::prelude::*;
use std::path;
use std::pin::Pin;

pub trait CredentialRepository {
    fn push(
        &self,
        credential: &Credential,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

    fn credentials(
        &self,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Credential>>> + Send>>;

    fn credential_of_id(
        &self,
        id: &CredentialID,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Credential>> + Send>>;
}

pub struct CredentialRepositoryDefaultImpl;

impl CredentialRepository for CredentialRepositoryDefaultImpl {
    fn push(
        &self,
        credential: &Credential,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let credential = credential.clone();
        let working_directory = working_directory.to_owned();
        future::lazy(move |_| {
            let id = credential.id_of_credential();
            let kind = credential.kind_of_credential();

            let credentials_path_buf = {
                let mut p = path::Path::new(&working_directory).to_path_buf();
                p.push(".iza");
                p.push("credential");
                p.push("credentials");
                p
            };

            let new_credentials = {
                let mut input_data = Vec::new();
                let mut credentials_file = fs::File::open(&credentials_path_buf)?;
                credentials_file.read_to_end(&mut input_data)?;
                let mut credentials: Vec<YamlCredential> = if input_data.is_empty() {
                    Vec::new()
                } else {
                    yaml::from_slice(&input_data)?
                };
                match credentials
                    .iter()
                    .find(|p| p.id_of_yaml_credential() == id.to_string())
                {
                    Some(_) => return Err(Error::AlreadyExistCredential),
                    None => { /* do nothing */ }
                }
                credentials.push(YamlCredential::new(id.to_string(), kind.to_string()));
                credentials
            };

            {
                let output_data = yaml::to_vec(&new_credentials)?;
                let mut credentials_file = fs::File::create(&credentials_path_buf)?;
                credentials_file.write(&output_data)?;
            }

            Ok(())
        })
        .boxed()
    }

    fn credentials(
        &self,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Credential>>> + Send>> {
        unimplemented!()
    }

    fn credential_of_id(
        &self,
        credential_id: &CredentialID,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Credential>> + Send>> {
        let credential_id = credential_id.clone();
        let working_directory = working_directory.to_owned();
        future::lazy(move |_| {
            let working_directory = working_directory.to_string();

            let credentials_path_buf = {
                let mut p = path::Path::new(&working_directory).to_path_buf();
                p.push(".iza");
                p.push("credential");
                p.push("credentials");
                p
            };

            {
                let mut input_data = Vec::new();
                let mut credentials_file = fs::File::open(&credentials_path_buf)?;
                credentials_file.read_to_end(&mut input_data)?;
                let credentials: Vec<YamlCredential> = if input_data.is_empty() {
                    Vec::new()
                } else {
                    yaml::from_slice(&input_data)?
                };
                let target_credential_id = credential_id.to_string();
                match credentials
                    .iter()
                    .find(|p| &p.id_of_yaml_credential() == &target_credential_id)
                {
                    Some(p) => Ok(Credential::restore(
                        p.id_of_yaml_credential(),
                        p.kind_of_yaml_credential(),
                    )?),
                    None => Err(Error::NotFoundCredential),
                }
            }
        })
        .boxed()
    }
}

pub trait HasCredentialRepository {
    type Repository: CredentialRepository;

    fn credential_repository(&self) -> &Self::Repository;
}
