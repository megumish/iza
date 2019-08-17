use crate::object::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait ObjectRepository {
    fn push(&self, object: &Object) -> Pin<Box<dyn Future<Output = Result<Object>> + Send>>;
}

pub trait HasObjectRepository {
    type Repository: ObjectRepository;

    fn object_repository(&self) -> &Self::Repository;
}
