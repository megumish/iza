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
        future::ready(Ok(ObjectInfo::new(local_path, remote_path, credential_id)))
            .and_then(move |o| self.object_info_repository().push(&o))
            .and_then(move |o| future::ready(Ok(Object::new(o.get_id(), package_id))))
            .and_then(move |o| self.object_repository().push(&o))
            .and_then(|_| future::ready(Ok(())))
            .boxed()
    }
}
