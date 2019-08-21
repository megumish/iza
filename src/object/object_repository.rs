mod yaml_object;

use self::yaml_object::*;
use crate::dot_iza::*;
use crate::object::*;
use futures::prelude::*;
use std::sync::Arc;

use crate::object::ResultFuture;

pub trait ObjectRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()>;

    fn push(
        &self,
        object: Arc<Object>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Object>>;

    fn delete(
        &self,
        object: Arc<Object>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Object>>;

    fn objects_of_package_id(
        &self,
        package_id: Arc<PackageID>,
        working_directory: &'static str,
    ) -> ResultFuture<Vec<Arc<Object>>>;
}

pub struct DotIzaObjectRepository;

const PRURAL_NAME: &'static str = "objects";

impl ObjectRepository for DotIzaObjectRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()> {
        init_module_file(working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn push(
        &self,
        object: Arc<Object>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Object>> {
        push_module::<_, YamlObject>(object, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn delete(
        &self,
        object: Arc<Object>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Object>> {
        delete_module::<_, YamlObject>(object, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn objects_of_package_id(
        &self,
        package_id: Arc<PackageID>,
        working_directory: &'static str,
    ) -> ResultFuture<Vec<Arc<Object>>> {
        modules_under_condition::<_, YamlObject, _>(
            move |o| &o.package_id_of_object() == &*package_id,
            working_directory,
            PRURAL_NAME,
        )
        .map_err(Into::into)
        .boxed()
    }
}

pub trait HasObjectRepository {
    type Repository: ObjectRepository;

    fn object_repository(&self) -> &Self::Repository;
}
