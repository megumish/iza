use crate::package::*;
use std::sync::{Arc, Mutex};

pub struct AppTestSuite {
    in_memory_executor_database: InMemoryExecutorDatabase,
}

impl HasExecutorRepository for AppTestSuite {
    type Repository = InMemoryExecutorDatabase;

    fn executor_repository(&self) -> &Self::Repository {
        &self.in_memory_executor_database
    }
}

#[test]
fn new_executor() {
    let app_test_suite = {
        let s = Box::new(AppTestSuite {
            in_memory_executor_database: InMemoryExecutorDatabase::new(),
        });
        Box::leak(s)
    };

    let new_executor_future = app_test_suite.new_executor("test_executor".to_string(), "");

    assert_eq!(
        futures::executor::ThreadPool::new()
            .expect("Future executor can't be initialized.")
            .run(new_executor_future)
            .expect("Should return Executor"),
        Arc::new(Executor::new("test_executor".to_string().into()))
    );

    let new_same_executor_future = app_test_suite.new_executor("test_executor".to_string(), "");

    assert_eq!(
        futures::executor::ThreadPool::new()
            .expect("Future executor can't be initialized.")
            .run(new_same_executor_future)
            .expect_err("Should disable to push new Executor that exists already")
            .kind(),
        &ErrorKind::AlreadyExistsExecutor
    );
}

#[test]
fn execute_by_empty_executor() {
    let target_executor = Arc::new(Executor::new("test_executor".to_string().into()));
    let app_test_suite = {
        let s = Box::new(AppTestSuite {
            in_memory_executor_database: InMemoryExecutorDatabase::new_with_initial_data(Arc::new(
                Mutex::new(vec![(&*target_executor).clone()]),
            )),
        });
        Box::leak(s)
    };

    let execute_future = app_test_suite.execute("test_executor".to_string(), "");

    assert_eq!(
        futures::executor::ThreadPool::new()
            .expect("Future executor can't be initialized.")
            .run(execute_future)
            .expect("The Executor should be enable to execute."),
        Vec::new()
    );
}
