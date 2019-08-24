use crate::object::*;
use futures::prelude::*;
use std::sync::Arc;

pub trait ObjectApp: HasObjectInfoRepository + HasObjectRepository + Sync {
    fn init(&'static self, working_directory: &'static str) -> ResultFuture<()> {
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
        working_directory: &'static str,
    ) -> ResultFuture<(Arc<Object>, Arc<ObjectInfo>)> {
        future::lazy(|_| Ok(ObjectInfo::new_arc(local_path, remote_path, credential_id)))
            .and_then(move |i| {
                future::try_join(
                    self.object_info_repository()
                        .push(i.clone(), working_directory),
                    future::lazy(move |_| Ok(Object::new_arc(i.id_of_object_info(), package_id))),
                )
            })
            .and_then(move |(i, o)| {
                future::try_join(
                    self.object_repository().push(o, working_directory),
                    future::lazy(move |_| Ok(i)),
                )
            })
            .boxed()
    }

    fn objects_of_package_id(
        &'static self,
        package_id: String,
        working_directory: &'static str,
    ) -> ResultFuture<Vec<(Arc<Object>, Arc<ObjectInfo>)>> {
        self.object_repository()
            .objects_of_package_id(Arc::new(package_id.into()), working_directory)
            .and_then(move |os| {
                let os = os.clone();
                future::try_join_all(os.into_iter().map(|o| {
                    let o2 = o.clone();
                    future::try_join(
                        future::lazy(move |_| Ok(o.clone())),
                        self.object_info_repository().object_info_of_id(
                            Arc::new(o2.object_info_id_of_object()),
                            working_directory,
                        ),
                    )
                }))
            })
            .boxed()
    }
}

pub trait HasObjectApp {
    type App: ObjectApp;

    fn object_app(&self) -> &Self::App;
}

impl<T> ObjectApp for T where T: HasObjectInfoRepository + HasObjectRepository + Sync {}
