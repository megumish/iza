use crate::package::*;
use std::sync::Arc;

pub trait PackageRepository {
    fn push(
        &'static self,
        package: Arc<Package>,
    ) -> Box<dyn Future<Item = Arc<Package>, Error = Error>>;

    fn add_command_to_package_of_name<PN, CID>(
        &'static self,
        package_name: PN,
        command_id: CID,
    ) -> Box<dyn Future<Item = Arc<Package>, Error = Error>>
    where
        PN: Into<PackageName>,
        CID: Into<CommandID>;
}

pub trait PackageRepositoryComponent {
    type Repository: PackageRepository;

    fn package_repository(&self) -> &Self::Repository;
}
