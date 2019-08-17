use crate::object::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait ObjectApp: HasObjectInfoRepository + HasObjectRepository + Sync {
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
            .and_then(move |o| future::ready(Ok(Object::new(o.get_id(), package_id))))
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
}
