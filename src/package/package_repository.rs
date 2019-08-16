use crate::package::*;

pub trait PackageRepository {
    fn push(&self, package: &Package) -> Result<()>;

    fn delete(&self, package: &Package) -> Result<()>;

    fn packages(&self, working_directory: &WorkingDirectory) -> Result<Vec<Package>>;

    fn package_of_name(
        &self,
        name: &PackageName,
        working_directory: &WorkingDirectory,
    ) -> Result<Package>;

    fn current_package(&self, working_directory: &WorkingDirectory) -> Result<Package>;

    fn set_current_package(&self, package: &Package) -> Result<()>;
}

pub trait HasPackageRepository {
    type Repository: PackageRepository;

    fn package_repository(&self) -> &Self::Repository;
}
