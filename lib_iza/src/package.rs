//! Package Domain
use futures::{future, prelude::*};
use std::sync::Arc;

/// Package App is a interface for library user.
pub trait PackageApp: PackageRepositoryComponent {
    /// new Package
    fn new_package<PN>(
        &'static self,
        package_name: PN,
    ) -> Box<dyn Future<Item = Arc<Package>, Error = Error>>
    where
        PN: Into<PackageName>,
    {
        Box::new(
            future::ok(Arc::new(Package::new(package_name)))
                .and_then(move |p| self.package_repository().push(p)),
        )
    }
}

/// Package is a unit of deployment.
pub struct Package {
    name: PackageName,
}

impl Package {
    fn new<PN>(package_name: PN) -> Self
    where
        PN: Into<PackageName>,
    {
        Self {
            name: package_name.into(),
        }
    }
}

/// Error of Package Domain
pub struct Error {}

mod package_name;
mod package_repository;

pub(self) use self::package_name::*;
pub(self) use self::package_repository::*;
