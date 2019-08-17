use crate::object::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait ObjectInfoRepository {
    fn push(
        &self,
        object_info: &ObjectInfo,
        working_directory: &str,
    ) -> Pin<Box<dyn Future<Output = Result<ObjectInfo>> + Send>>;
}

pub trait HasObjectInfoRepository {
    type Repository: ObjectInfoRepository;

    fn object_info_repository(&self) -> &Self::Repository;
}
