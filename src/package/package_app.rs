use crate::package::*;
use futures::prelude::*;
use std::pin::Pin;

pub trait PackageApp: HasPackageRepository + Sync {
    fn new_package(
        &'static self,
        name: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>>>> {
        future::ready(Ok(Package::new(name, working_directory)))
            .and_then(move |p| future::ready(self.package_repository().push(&p)))
            .boxed()
    }

    fn packages(
        &'static self,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Package>>>>> {
        future::ready(
            self.package_repository()
                .packages(&working_directory.into()),
        )
        .boxed()
    }

    fn current_package(
        &'static self,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<Package>>>> {
        future::ready(
            self.package_repository()
                .current_package(&working_directory.into()),
        )
        .boxed()
    }

    fn switch_current_package(
        &'static self,
        name: String,
        working_directory: String,
    ) -> Pin<Box<dyn Future<Output = Result<()>>>> {
        future::ready(
            self.package_repository()
                .package_of_name(&name.into(), &working_directory.into()),
        )
        .and_then(move |p| future::ready(self.package_repository().set_current_package(&p)))
        .boxed()
    }
}

pub trait HasPackageApp {
    type App: PackageApp;

    fn package_app(&self) -> &Self::App;
}

impl<T> PackageApp for T where T: HasPackageRepository + Sync {}
