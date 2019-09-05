mod executor_app;

use crate::package::*;
use std::sync::{Arc, Mutex};

impl IntegrationTestSuite {
    pub fn new_with_initial_data(init_executors: Arc<Mutex<Vec<Executor>>>) -> Self {
        IntegrationTestSuite {
            executor_app_suite: executor_app::AppTestSuite::new_with_initial_data(init_executors),
        }
    }
}

pub struct IntegrationTestSuite {
    executor_app_suite: executor_app::AppTestSuite,
}

impl HasExecutorRepository for IntegrationTestSuite {
    type Repository = InMemoryExecutorDatabase;

    fn executor_repository(&self) -> &Self::Repository {
        self.executor_app_suite.executor_repository()
    }
}

impl IntegrationTestSuite {
    pub fn new() -> Self {
        Self {
            executor_app_suite: executor_app::AppTestSuite::new(),
        }
    }
}
