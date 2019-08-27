mod executor_app;

use crate::package::*;

pub struct IntegrationTestSuite {
    executor_app_suite: executor_app::AppTestSuite,
}

impl HasExecutorRepository for IntegrationTestSuite {
    type Repository = InMemoryExecutorDatabase;

    fn executor_repository(&self) -> &Self::Repository {
        self.executor_app_suite.executor_repository()
    }
}
