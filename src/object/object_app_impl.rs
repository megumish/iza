use crate::object::*;

pub struct ObjectAppImpl;

impl HasObjectRepository for ObjectAppImpl {
    type Repository = DotIzaObjectRepository;

    fn object_repository(&self) -> &Self::Repository {
        &DotIzaObjectRepository
    }
}

impl HasObjectInfoRepository for ObjectAppImpl {
    type Repository = DotIzaObjectInfoRepository;

    fn object_info_repository(&self) -> &Self::Repository {
        &DotIzaObjectInfoRepository
    }
}
