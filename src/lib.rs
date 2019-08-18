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

use crate::{credential::*, object::*, package::*, ssh_connection::*, system_directory::*};

pub struct AppSuite {
    system_directory_app: SystemDirectoryAppImpl,
    package_app: PackageAppImpl,
    credential_app: CredentialAppImpl,
    ssh_connection_app: SSHConnectionAppImpl,
    object_app: ObjectAppImpl,
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

impl HasCredentialApp for AppSuite {
    type App = CredentialAppImpl;

    fn credential_app(&self) -> &Self::App {
        &self.credential_app
    }
}

impl HasSSHConnectionApp for AppSuite {
    type App = SSHConnectionAppImpl;

    fn ssh_connection_app(&self) -> &Self::App {
        &self.ssh_connection_app
    }
}

impl HasObjectApp for AppSuite {
    type App = ObjectAppImpl;

    fn object_app(&self) -> &Self::App {
        &self.object_app
    }
}

impl AppSuite {
    pub fn new() -> Self {
        let system_directory_app = SystemDirectoryAppImpl;
        let package_app = PackageAppImpl;
        let credential_app = CredentialAppImpl;
        let ssh_connection_app = SSHConnectionAppImpl;
        let object_app = ObjectAppImpl;
        Self {
            system_directory_app,
            package_app,
            credential_app,
            ssh_connection_app,
            object_app,
        }
    }
}
