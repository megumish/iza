use crate::object::*;

pub struct ObjectAppImpl;

impl HasObjectRepository for ObjectAppImpl {
    type Repository = ObjectRepositoryDefaultImpl;

    fn object_repository(&self) -> &Self::Repository {
        &ObjectRepositoryDefaultImpl
    }
}

impl HasObjectInfoRepository for ObjectAppImpl {
    type Repository = ObjectInfoRepositoryDefaultImpl;

    fn object_info_repository(&self) -> &Self::Repository {
        &ObjectInfoRepositoryDefaultImpl
    }
}
