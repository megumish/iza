use crate::package::*;

pub trait PackageRepository {
    fn push(&self, package: &Package) -> Result<()>;

    fn delete(&self, package: &Package) -> Result<()>;
}

pub trait HasPackageRepository {
    type Repository: PackageRepository;

    fn package_repository(&self) -> &Self::Repository;
}
