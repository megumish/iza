use crate::system_directory::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait SystemDirectoryMaker {
    fn make_top_directory(
        &self,
        working_directory: &String,
    ) -> Pin<Box<dyn Future<Output = Result<String>> + Send>>;

    fn make_package_system(
        &self,
        top_directory: &String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

    fn make_object_system(
        &self,
        top_directory: &String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;

    fn make_credential_system(
        &self,
        top_directory: &String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
}

pub trait HasSystemDirectoryMaker {
    type Service: SystemDirectoryMaker;

    fn system_directory_maker(&self) -> &Self::Service;
}
