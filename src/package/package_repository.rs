mod yaml_package;

use self::yaml_package::*;
use crate::dot_iza::*;
use crate::package::*;
use futures::prelude::*;
use std::sync::Arc;

use crate::package::ResultFuture;

pub trait PackageRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()>;

    fn push(
        &self,
        package: Arc<Package>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Package>>;

    fn delete(
        &self,
        package: Arc<Package>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Package>>;

    fn packages(&self, working_directory: &'static str) -> ResultFuture<Vec<Package>>;
}

pub struct DotIzaPackageRepository;

const PRURAL_NAME: &'static str = "packages";

impl PackageRepository for DotIzaPackageRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()> {
        init_module_file(working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn push(
        &self,
        package: Arc<Package>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Package>> {
        push_module::<_, YamlPackage>(package, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn delete(
        &self,
        package: Arc<Package>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Package>> {
        delete_module::<_, YamlPackage>(package, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }

    fn packages(&self, working_directory: &'static str) -> ResultFuture<Vec<Package>> {
        modules_under_condition::<_, YamlPackage, _>(|_| true, working_directory, PRURAL_NAME)
            .map_err(Into::into)
            .boxed()
    }
}

pub trait HasPackageRepository {
    type Repository: PackageRepository;

    fn package_repository(&self) -> &Self::Repository;
}
