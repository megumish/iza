use crate::package::tests as package_testing;
use crate::package::*;
use crate::wording::*;
use std::sync::{Arc, Mutex};

impl AppTestSuite {
    fn new() -> Self {
        Self {
            package_suite: package_testing::IntegrationTestSuite::new(),
            in_memory_execution_wording_database: InMemoryExecutionWordingDatabase::new(),
            in_memory_executor_wording_database: InMemoryExecutorWordingDatabase::new(),
        }
    }
}

pub struct AppTestSuite {
    package_suite: package_testing::IntegrationTestSuite,
    in_memory_execution_wording_database: InMemoryExecutionWordingDatabase,
    in_memory_executor_wording_database: InMemoryExecutorWordingDatabase,
}

impl HasExecutorApp for AppTestSuite {
    type App = package_testing::IntegrationTestSuite;

    fn executor_app(&self) -> &Self::App {
        &self.package_suite
    }
}

impl HasExecutionWordingRepository for AppTestSuite {
    type Repository = InMemoryExecutionWordingDatabase;

    fn execution_wording_repository(&self) -> &Self::Repository {
        &self.in_memory_execution_wording_database
    }
}

impl HasExecutorWordingRepository for AppTestSuite {
    type Repository = InMemoryExecutorWordingDatabase;

    fn executor_wording_repository(&self) -> &Self::Repository {
        &self.in_memory_executor_wording_database
    }
}

#[test]
fn new_executor_with_wording() {
    let app_test_suite = {
        let s = Box::new(AppTestSuite::new());
        Box::leak(s)
    };

    let new_executor_with_wording_future =
        app_test_suite.new_executor_with_wording("test_executor".to_string(), "");

    assert_eq!(
        futures::executor::ThreadPool::new()
            .expect("Future executor can't be initialized.")
            .run(new_executor_with_wording_future)
            .expect("Should return Executor Wording"),
        Arc::new(ExecutorWording {
            executor_name: "test_executor".to_string().into()
        })
    );
}

#[test]
fn deploy_with_wording() {
    let target_executor = Arc::new(Executor::new("test_executor".to_string().into()));
    let app_test_suite = {
        let s = Box::new(AppTestSuite {
            package_suite: package_testing::IntegrationTestSuite::new_with_initial_data(Arc::new(
                Mutex::new(vec![(&*target_executor).clone()]),
            )),
            in_memory_execution_wording_database: InMemoryExecutionWordingDatabase::new(),
            in_memory_executor_wording_database: InMemoryExecutorWordingDatabase::new(),
        });
        Box::leak(s)
    };

    let deploy_with_wording_future =
        app_test_suite.deploy_with_wording("test_executor".to_string(), "");

    assert_eq!(
        futures::executor::ThreadPool::new()
            .expect("Future executor can't be initialized.")
            .run(deploy_with_wording_future)
            .expect("The Executor should be deploy to execute."),
        Vec::new()
    );
}
