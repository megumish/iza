use crate::package::tests as package_testing;
use crate::package::*;
use crate::wording::*;

pub struct AppTestSuite {
    package_suite: package_testing::IntegrationTestSuite,
}

impl HasExecutorApp for AppTestSuite {
    type App = package_testing::IntegrationTestSuite;

    fn executor_app(&self) -> &Self::App {
        &self.package_suite
    }
}
