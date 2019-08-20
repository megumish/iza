use crate::package::*;

pub struct PackageAppImpl;

impl HasPackageRepository for PackageAppImpl {
    type Repository = DotIzaPackageRepository;

    fn package_repository(&self) -> &Self::Repository {
        &DotIzaPackageRepository
    }
}
