use crate::package::*;
use futures::prelude::*;
use std::sync::Arc;

pub trait PackageApp: HasPackageRepository + Sync {
    fn init(&'static self, working_directory: &'static str) -> ResultFuture<()> {
        self.package_repository().init(working_directory).boxed()
    }

    fn new_package(
        &'static self,
        name: String,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Package>> {
        future::lazy(|_| Ok(Package::new_arc(name)))
            .and_then(move |p| self.package_repository().push(p, working_directory))
            .boxed()
    }

    fn delete_package(
        &'static self,
        name: String,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Package>> {
        future::lazy(|_| Ok(Package::new_arc(name)))
            .and_then(move |p| self.package_repository().delete(p, working_directory))
            .boxed()
    }

    fn packages(&'static self, working_directory: &'static str) -> ResultFuture<Vec<Package>> {
        self.package_repository()
            .packages(working_directory)
            .boxed()
    }
}

pub trait HasPackageApp {
    type App: PackageApp;

    fn package_app(&self) -> &Self::App;
}

impl<T> PackageApp for T where T: HasPackageRepository + Sync {}
