use crate::package::*;
use std::sync::Arc;

pub trait PackageRepository {
    fn push(
        &'static self,
        package: Arc<Package>,
    ) -> Box<dyn Future<Item = Arc<Package>, Error = Error>>;
}

pub trait PackageRepositoryComponent {
    type Repository: PackageRepository;

    fn package_repository(&self) -> &Self::Repository;
}
