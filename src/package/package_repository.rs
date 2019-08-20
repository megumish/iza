mod yaml_package;

use self::yaml_package::*;
use crate::dot_iza::*;
use crate::package::*;
use futures::prelude::*;
use std::sync::Arc;

use crate::package::{Result, ResultFuture};

pub trait PackageRepository {
    fn init(&self, working_directory: &'static str) -> ResultFuture<()>;

    fn push(
        &self,
        package: Arc<Package>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Package>>;

    fn delete(&self, package: &Package, working_directory: &WorkingDirectory) -> Result<()>;

    fn packages(&self, working_directory: &WorkingDirectory) -> Result<Vec<Package>>;

    fn package_of_name(
        &self,
        name: &PackageName,
        working_directory: &WorkingDirectory,
    ) -> Result<Package>;

    fn current_package(&self, working_directory: &WorkingDirectory) -> Result<Package>;

    fn set_current_package(
        &self,
        package: &Package,
        working_directory: &WorkingDirectory,
    ) -> Result<()>;
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

    fn delete(&self, _: &Package, _: &WorkingDirectory) -> Result<()> {
        unimplemented!()
        // let name = package.name_of_package().to_string();
        // let working_directory = working_directory.to_string();

        // let packages_path_buf = {
        //     let mut p = path::Path::new(&working_directory).to_path_buf();
        //     p.push(".iza");
        //     p.push("package");
        //     p.push("packages");
        //     p
        // };

        // {
        //     let mut input_data = Vec::new();
        //     let mut packages_file = fs::File::open(&packages_path_buf)?;
        //     packages_file.read_to_end(&mut input_data)?;
        //     let packages: Vec<YamlPackage> = if input_data.is_empty() {
        //         Vec::new()
        //     } else {
        //         yaml::from_slice(&input_data)?
        //     };
        //     let remove_package = YamlPackage::new(name);
        //     let new_packages: Vec<&YamlPackage> =
        //         packages.iter().filter(|p| **p != remove_package).collect();
        //     let output_data = yaml::to_vec(&new_packages)?;
        //     let mut packages_file = fs::File::create(&packages_path_buf)?;
        //     packages_file.write(&output_data)?;
        // }

        // Ok(())
    }

    fn packages(&self, _: &WorkingDirectory) -> Result<Vec<Package>> {
        unimplemented!()
        // let working_directory = working_directory.to_string();

        // let packages_path_buf = {
        //     let mut p = path::Path::new(&working_directory).to_path_buf();
        //     p.push(".iza");
        //     p.push("package");
        //     p.push("packages");
        //     p
        // };

        // Ok({
        //     let mut input_data = Vec::new();
        //     let mut packages_file = fs::File::open(&packages_path_buf)?;
        //     packages_file.read_to_end(&mut input_data)?;
        //     let packages: Vec<YamlPackage> = if input_data.is_empty() {
        //         Vec::new()
        //     } else {
        //         yaml::from_slice(&input_data)?
        //     };
        //     packages
        //         .iter()
        //         .map(|p| Package::new(p.name_of_yaml_package()))
        //         .collect()
        // })
    }

    fn package_of_name(&self, _: &PackageName, _: &WorkingDirectory) -> Result<Package> {
        unimplemented!()
        // let working_directory = working_directory.to_string();

        // let packages_path_buf = {
        //     let mut p = path::Path::new(&working_directory).to_path_buf();
        //     p.push(".iza");
        //     p.push("package");
        //     p.push("packages");
        //     p
        // };

        // {
        //     let mut input_data = Vec::new();
        //     let mut packages_file = fs::File::open(&packages_path_buf)?;
        //     packages_file.read_to_end(&mut input_data)?;
        //     let packages: Vec<YamlPackage> = if input_data.is_empty() {
        //         Vec::new()
        //     } else {
        //         yaml::from_slice(&input_data)?
        //     };
        //     let target_package = YamlPackage::new(name.to_string());
        //     match packages.iter().find(|p| **p == target_package) {
        //         Some(p) => Ok(Package::new(p.name_of_yaml_package())),
        //         None => Err(Error::NotFoundPackage),
        //     }
        // }
    }

    fn current_package(&self, _: &WorkingDirectory) -> Result<Package> {
        unimplemented!()
        // let working_directory = working_directory.to_string();

        // let packages_path_buf = {
        //     let mut p = path::Path::new(&working_directory).to_path_buf();
        //     p.push(".iza");
        //     p.push("package");
        //     p.push("current");
        //     p
        // };

        // {
        //     let mut input_data = Vec::new();
        //     let mut packages_file = fs::File::open(&packages_path_buf)?;
        //     packages_file.read_to_end(&mut input_data)?;
        //     let package: YamlPackage = yaml::from_slice(&input_data)?;
        //     Ok(Package::new(package.name_of_yaml_package()))
        // }
    }

    fn set_current_package(&self, _: &Package, _: &WorkingDirectory) -> Result<()> {
        unimplemented!()
        // let working_directory = working_directory.to_string();

        // let packages_path_buf = {
        //     let mut p = path::Path::new(&working_directory).to_path_buf();
        //     p.push(".iza");
        //     p.push("package");
        //     p.push("current");
        //     p
        // };

        // {
        //     let yaml_package = YamlPackage::new(package.name_of_package());
        //     let output_data = yaml::to_vec(&yaml_package)?;
        //     let mut packages_file = fs::File::create(&packages_path_buf)?;
        //     packages_file.write(&output_data)?;
        //     Ok(())
        // }
    }
}

pub trait HasPackageRepository {
    type Repository: PackageRepository;

    fn package_repository(&self) -> &Self::Repository;
}
