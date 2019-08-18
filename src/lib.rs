#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[cfg(feature = "exec")]
#[macro_use]
extern crate lazy_static;

pub mod credential;
pub mod object;
pub mod package;
pub mod ssh_connection;
pub mod system_directory;

#[cfg(feature = "exec")]
lazy_static! {
    pub static ref SUITE: AppSuite = AppSuite::new();
}

use crate::{package::*, system_directory::*};

pub struct AppSuite {
    system_directory_app: SystemDirectoryAppImpl,
    package_app: PackageAppImpl,
}

impl HasSystemDirectoryApp for AppSuite {
    type App = SystemDirectoryAppImpl;

    fn system_directory_app(&self) -> &Self::App {
        &self.system_directory_app
    }
}

impl HasPackageApp for AppSuite {
    type App = PackageAppImpl;

    fn package_app(&self) -> &Self::App {
        &self.package_app
    }
}

impl AppSuite {
    pub fn new() -> Self {
        let system_directory_app = SystemDirectoryAppImpl;
        let package_app = PackageAppImpl;
        Self {
            system_directory_app,
            package_app,
        }
    }
}
