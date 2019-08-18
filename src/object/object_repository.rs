mod yaml_object;

use self::yaml_object::*;
use crate::dot_iza::*;
use crate::object::*;
use futures::prelude::*;
use serde_yaml as yaml;
use std::fs;
use std::io::prelude::*;
use std::path;
use std::pin::Pin;

pub trait ObjectRepository: DotIza {
    fn init(&self, working_directory: &'static str) -> RetFuture<()>;

    fn push(
        &self,
        object: &Object,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Object>> + Send>>;

    fn objects_of_package_id(
        &self,
        package_id: &PackageID,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Object>>> + Send>>;
}

pub struct ObjectRepositoryDefaultImpl;

impl DotIza for ObjectRepositoryDefaultImpl {
    type Module = Object;
    type YamlModule = YamlObject;
    type Error = Error;
    const MODULE_NAME: &'static str = "object";
    const MODULE_PRURAL_NAME: &'static str = "objects";
}

impl ObjectRepository for ObjectRepositoryDefaultImpl {
    fn init(&self, working_directory: &'static str) -> RetFuture<()> {
        Self::init_module_top(working_directory)
            .and_then(|t| Self::init_module_files(t))
            .boxed()
    }

    fn push(
        &self,
        object: &Object,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Object>> + Send>> {
        let object = object.clone();
        let working_directory = working_directory.to_owned();
        future::lazy(move |_| {
            let id = object.id_of_object();
            let package_id = object.package_id_of_object();
            let object_info_id = object.object_info_id_of_object();

            let objects_path_buf = {
                let mut p = path::Path::new(&working_directory).to_path_buf();
                p.push(".iza");
                p.push("object");
                p.push("objects");
                p
            };

            let new_objects = {
                let mut input_data = Vec::new();
                let mut objects_file = fs::File::open(&objects_path_buf)?;
                objects_file.read_to_end(&mut input_data)?;
                let mut objects: Vec<YamlObject> = if input_data.is_empty() {
                    Vec::new()
                } else {
                    yaml::from_slice(&input_data)?
                };
                let new_yaml_object = YamlObject::new(
                    id.to_string(),
                    package_id.to_string(),
                    object_info_id.to_string(),
                );
                match objects.iter().find(|p| *p == &new_yaml_object) {
                    Some(_) => return Err(Error::AlreadyExistObject),
                    None => { /* do nothing */ }
                }
                objects.push(new_yaml_object);
                objects
            };

            {
                let output_data = yaml::to_vec(&new_objects)?;
                let mut objects_file = fs::File::create(&objects_path_buf)?;
                objects_file.write(&output_data)?;
            }

            Ok(object.clone())
        })
        .boxed()
    }

    fn objects_of_package_id(
        &self,
        package_id: &PackageID,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Object>>> + Send>> {
        let package_id = package_id.clone();
        let working_directory = working_directory.to_owned();
        future::lazy(move |_| {
            let working_directory = working_directory.to_string();

            let objects_path_buf = {
                let mut p = path::Path::new(&working_directory).to_path_buf();
                p.push(".iza");
                p.push("object");
                p.push("objects");
                p
            };

            {
                let mut input_data = Vec::new();
                let mut objects_file = fs::File::open(&objects_path_buf)?;
                objects_file.read_to_end(&mut input_data)?;
                let objects: Vec<YamlObject> = if input_data.is_empty() {
                    Vec::new()
                } else {
                    yaml::from_slice(&input_data)?
                };
                let target_package_id = package_id.to_string();
                objects
                    .iter()
                    .filter(|p| &p.id_of_yaml_object() != &target_package_id)
                    .map(|p| {
                        Ok(Object::restore(
                            p.id_of_yaml_object(),
                            p.object_info_id_of_yaml_object(),
                            p.package_id_of_yaml_object(),
                        ))
                    })
                    .collect()
            }
        })
        .boxed()
    }
}

pub trait HasObjectRepository {
    type Repository: ObjectRepository;

    fn object_repository(&self) -> &Self::Repository;
}
