use crate::package::*;

pub struct PackageAppImpl;

impl HasPackageRepository for PackageAppImpl {
    type Repository = PackageRepositoryDefaultImpl;

    fn package_repository(&self) -> &Self::Repository {
        &PackageRepositoryDefaultImpl
    }
}
