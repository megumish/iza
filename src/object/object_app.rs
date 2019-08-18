use crate::object::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait ObjectApp: HasObjectInfoRepository + HasObjectRepository + Sync {
    fn init(&'static self, working_directory: &'static str) -> RetFuture<()> {
        future::try_join(
            self.object_repository().init(working_directory),
            self.object_info_repository().init(working_directory),
        )
        .and_then(|_| future::ready(Ok(())))
        .boxed()
    }

    fn new_object(
        &'static self,
        local_path: String,
        remote_path: String,
        credential_id: String,
        package_id: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        let working_directory2 = working_directory.clone();
        future::ready(Ok(ObjectInfo::new(local_path, remote_path, credential_id)))
            .and_then(move |o| self.object_info_repository().push(&o, &working_directory))
            .and_then(move |o| future::ready(Ok(Object::new(o.id_of_object_info(), package_id))))
            .and_then(move |o| self.object_repository().push(&o, &working_directory2))
            .and_then(|_| future::ready(Ok(())))
            .boxed()
    }

    fn objects_of_package_id(
        &'static self,
        package_id: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Object>>> + Send>> {
        self.object_repository()
            .objects_of_package_id(&package_id.into(), &working_directory)
            .boxed()
    }

    fn object_info_of_id(
        &'static self,
        object_info_id: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<ObjectInfo>> + Send>> {
        self.object_info_repository()
            .object_info_of_id(&object_info_id.into(), &working_directory)
            .boxed()
    }
}

pub trait HasObjectApp {
    type App: ObjectApp;

    fn object_app(&self) -> &Self::App;
}

impl<T> ObjectApp for T where T: HasObjectInfoRepository + HasObjectRepository + Sync {}
