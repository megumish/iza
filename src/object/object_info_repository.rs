mod yaml_object_info;

use self::yaml_object_info::*;
use crate::object::*;
use futures::prelude::*;
use serde_yaml as yaml;
use std::fs;
use std::io::prelude::*;
use std::path;
use std::pin::Pin;

pub trait ObjectInfoRepository {
    fn push(
        &self,
        object_info: &ObjectInfo,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<ObjectInfo>> + Send>>;
}

pub struct ObjectInfoRepositoryDefaultImpl;

impl ObjectInfoRepository for ObjectInfoRepositoryDefaultImpl {
    fn push(
        &self,
        object_info: &ObjectInfo,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<ObjectInfo>> + Send>> {
        let object_info = object_info.clone();
        let working_directory = working_directory.to_owned();
        future::lazy(move |_| {
            let id = object_info.id_of_object_info();
            let local_path = object_info.local_path_of_object_info();
            let remote_path = object_info.remote_path_of_object_info();
            let credential_id = object_info.credential_id_of_object_info();

            let object_info_path_buf = {
                let mut p = path::Path::new(&working_directory).to_path_buf();
                p.push(".iza");
                p.push("object");
                p.push("object_info");
                p
            };

            let new_object_info = {
                let mut input_data = Vec::new();
                let mut object_info_file = fs::File::open(&object_info_path_buf)?;
                object_info_file.read_to_end(&mut input_data)?;
                let mut object_info: Vec<YamlObjectInfo> = if input_data.is_empty() {
                    Vec::new()
                } else {
                    yaml::from_slice(&input_data)?
                };
                let new_yaml_object_info = YamlObjectInfo::new(
                    id.to_string(),
                    local_path.to_string(),
                    remote_path.to_string(),
                    credential_id.to_string(),
                );
                match object_info.iter().find(|p| *p == &new_yaml_object_info) {
                    Some(_) => return Err(Error::AlreadyExistObjectInfo),
                    None => { /* do nothing */ }
                }
                object_info.push(new_yaml_object_info);
                object_info
            };

            {
                let output_data = yaml::to_vec(&new_object_info)?;
                let mut object_info_file = fs::File::create(&object_info_path_buf)?;
                object_info_file.write(&output_data)?;
            }

            Ok(object_info.clone())
        })
        .boxed()
    }
}

pub trait HasObjectInfoRepository {
    type Repository: ObjectInfoRepository;

    fn object_info_repository(&self) -> &Self::Repository;
}
