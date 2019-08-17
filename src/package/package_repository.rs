mod toml_package;

use self::toml_package::*;
use crate::package::*;
use std::fs;
use std::io::prelude::*;
use std::path;

pub trait PackageRepository {
    fn push(&self, package: &Package, working_directory: &WorkingDirectory) -> Result<()>;

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

struct PackageRepositoryDefaultImpl;

impl PackageRepository for PackageRepositoryDefaultImpl {
    fn push(&self, package: &Package, working_directory: &WorkingDirectory) -> Result<()> {
        let name = package.name_of_package().to_string();
        let working_directory = working_directory.to_string();

        let packages_path_buf = {
            let mut p = path::Path::new(&working_directory).to_path_buf();
            p.push(".iza");
            p.push("package");
            p.push("packages");
            p
        };

        let new_packages = {
            let mut input_data = Vec::new();
            let mut packages_file = fs::File::open(&packages_path_buf)?;
            packages_file.read(&mut input_data)?;
            let mut packages: Vec<TomlPackage> = toml::from_slice(&input_data)?;
            match packages
                .iter()
                .find(|p| **p == TomlPackage::new(name.clone()))
            {
                Some(_) => return Err(Error::AlreadyExistPackage),
                None => { /* do nothing */ }
            }
            packages.push(TomlPackage::new(name));
            packages
        };

        {
            let output_data = toml::to_vec(&new_packages)?;
            let mut packages_file = fs::File::create(&packages_path_buf)?;
            packages_file.write(&output_data)?;
        }

        Ok(())
    }

    fn delete(&self, package: &Package, working_directory: &WorkingDirectory) -> Result<()> {
        let name = package.name_of_package().to_string();
        let working_directory = working_directory.to_string();

        let packages_path_buf = {
            let mut p = path::Path::new(&working_directory).to_path_buf();
            p.push(".iza");
            p.push("package");
            p.push("packages");
            p
        };

        {
            let mut input_data = Vec::new();
            let mut packages_file = fs::File::open(&packages_path_buf)?;
            packages_file.read(&mut input_data)?;
            let packages: Vec<TomlPackage> = toml::from_slice(&input_data)?;
            let remove_package = TomlPackage::new(name);
            let new_packages: Vec<&TomlPackage> =
                packages.iter().filter(|p| **p != remove_package).collect();
            let output_data = toml::to_vec(&new_packages)?;
            let mut packages_file = fs::File::create(&packages_path_buf)?;
            packages_file.write(&output_data)?;
        }

        Ok(())
    }

    fn packages(&self, working_directory: &WorkingDirectory) -> Result<Vec<Package>> {
        let working_directory = working_directory.to_string();

        let packages_path_buf = {
            let mut p = path::Path::new(&working_directory).to_path_buf();
            p.push(".iza");
            p.push("package");
            p.push("packages");
            p
        };

        Ok({
            let mut input_data = Vec::new();
            let mut packages_file = fs::File::open(&packages_path_buf)?;
            packages_file.read(&mut input_data)?;
            let packages: Vec<TomlPackage> = toml::from_slice(&input_data)?;
            packages
                .iter()
                .map(|p| Package::new(p.name_of_toml_package()))
                .collect()
        })
    }

    fn package_of_name(
        &self,
        name: &PackageName,
        working_directory: &WorkingDirectory,
    ) -> Result<Package> {
        let working_directory = working_directory.to_string();

        let packages_path_buf = {
            let mut p = path::Path::new(&working_directory).to_path_buf();
            p.push(".iza");
            p.push("package");
            p.push("packages");
            p
        };

        {
            let mut input_data = Vec::new();
            let mut packages_file = fs::File::open(&packages_path_buf)?;
            packages_file.read(&mut input_data)?;
            let packages: Vec<TomlPackage> = toml::from_slice(&input_data)?;
            let target_package = TomlPackage::new(name.to_string());
            match packages.iter().find(|p| **p == target_package) {
                Some(p) => Ok(Package::new(p.name_of_toml_package())),
                None => Err(Error::NotFoundPackage),
            }
        }
    }

    fn current_package(&self, working_directory: &WorkingDirectory) -> Result<Package> {
        let working_directory = working_directory.to_string();

        let packages_path_buf = {
            let mut p = path::Path::new(&working_directory).to_path_buf();
            p.push(".iza");
            p.push("package");
            p.push("current");
            p
        };

        {
            let mut input_data = Vec::new();
            let mut packages_file = fs::File::open(&packages_path_buf)?;
            packages_file.read(&mut input_data)?;
            let package: TomlPackage = toml::from_slice(&input_data)?;
            Ok(Package::new(package.name_of_toml_package()))
        }
    }

    fn set_current_package(
        &self,
        package: &Package,
        working_directory: &WorkingDirectory,
    ) -> Result<()> {
        let working_directory = working_directory.to_string();

        let packages_path_buf = {
            let mut p = path::Path::new(&working_directory).to_path_buf();
            p.push(".iza");
            p.push("package");
            p.push("current");
            p
        };

        {
            let toml_package = TomlPackage::new(package.name_of_package());
            let output_data = toml::to_vec(&toml_package)?;
            let mut packages_file = fs::File::create(&packages_path_buf)?;
            packages_file.write(&output_data)?;
            Ok(())
        }
    }
}

pub trait HasPackageRepository {
    type Repository: PackageRepository;

    fn package_repository(&self) -> &Self::Repository;
}
