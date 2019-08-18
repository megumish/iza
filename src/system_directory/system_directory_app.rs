use crate::system_directory::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait SystemDirectoryApp: HasSystemDirectoryMaker + Sync {
    fn new_system_directory(
        &'static self,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        self.system_directory_maker()
            .make_top_directory(&working_directory)
            .and_then(move |t| {
                future::try_join3(
                    self.system_directory_maker().make_package_system(&t),
                    self.system_directory_maker().make_object_system(&t),
                    self.system_directory_maker().make_credential_system(&t),
                )
            })
            .and_then(|_| future::ready(Ok(())))
            .boxed()
    }
}

pub trait HasSystemDirectoryApp {
    type App: SystemDirectoryApp;

    fn system_directory_app(&self) -> &Self::App;
}

impl<T> SystemDirectoryApp for T where T: HasSystemDirectoryMaker + Sync {}
