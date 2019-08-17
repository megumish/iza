use crate::object::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait ObjectRepository {
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

pub trait HasObjectRepository {
    type Repository: ObjectRepository;

    fn object_repository(&self) -> &Self::Repository;
}
