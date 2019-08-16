use crate::package::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait PackageApp: HasPackageRepository + Sync {
    fn new_package(
        &'static self,
        name: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>>>> {
        future::ready(Package::new(name, working_directory))
            .map(move |p| self.package_repository().push(&p))
            .boxed()
    }
}

pub trait HasPackageApp {
    type App: PackageApp;

    fn package_app(&self) -> &Self::App;
}

impl<T> PackageApp for T where T: HasPackageRepository + Sync {}
