use crate::package::*;

pub trait PackageApp: HasPackageRepository {
    fn new_package(&self, name: String, working_directory: String) -> Result<()> {
        Ok(Package::new(name, working_directory)).and_then(|p| self.package_repository().push(&p))
    }
}

pub trait HasPackageApp {
    type App: PackageApp;

    fn package_app(&self) -> &Self::App;
}

impl<T> PackageApp for T where T: HasPackageRepository {}
