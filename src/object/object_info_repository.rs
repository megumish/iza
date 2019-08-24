mod yaml_object_info;

use self::yaml_object_info::*;
use crate::dot_iza::*;
use crate::object::*;
use futures::prelude::*;
use std::sync::Arc;

use crate::object::ResultFuture;

pub trait ObjectInfoRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()>;

    fn push(
        &self,
        object_info: Arc<ObjectInfo>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<ObjectInfo>>;

    fn delete(
        &self,
        object_info: Arc<ObjectInfo>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<ObjectInfo>>;

    fn object_info_of_id(
        &self,
        id: Arc<ObjectInfoID>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<ObjectInfo>>;
}

pub struct DotIzaObjectInfoRepository;

const PRURAL_NAME: &'static str = "object_info";

impl ObjectInfoRepository for DotIzaObjectInfoRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()> {
        init_module_file(working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn push(
        &self,
        object_info: Arc<ObjectInfo>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<ObjectInfo>> {
        push_module::<_, YamlObjectInfo>(object_info, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn delete(
        &self,
        object_info: Arc<ObjectInfo>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<ObjectInfo>> {
        delete_module::<_, YamlObjectInfo>(object_info, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn object_info_of_id(
        &self,
        id: Arc<ObjectInfoID>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<ObjectInfo>> {
        first_module_under_condition::<_, YamlObjectInfo, _>(
            move |i| &i.id_of_object_info() == &*id,
            working_directory,
            PRURAL_NAME,
        )
        .map_err(Into::into)
        .boxed()
    }
}

pub trait HasObjectInfoRepository {
    type Repository: ObjectInfoRepository;

    fn object_info_repository(&self) -> &Self::Repository;
}
