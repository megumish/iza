use crate::package::*;

pub trait PackageApp: HasPackageRepository {}

pub trait HasPackageApp {
    type App: PackageApp;

    fn package_app(&self) -> &Self::App;
}

impl<T> PackageApp for T where T: HasPackageRepository {}
