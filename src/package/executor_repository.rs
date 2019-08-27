use crate::package::*;
use std::sync::Arc;

#[cfg(test)]
use futures::prelude::*;
#[cfg(test)]
use std::sync::Mutex;

pub trait ExecutorRepository {
    fn push(
        &self,
        executor: Arc<Executor>,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Executor>>;

    fn executor_of_name(
        &self,
        name: ExecutorName,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Executor>>;
}

pub trait HasExecutorRepository {
    type Repository: ExecutorRepository;

    fn executor_repository(&self) -> &Self::Repository;
}

#[cfg(test)]
pub struct InMemoryExecutorDatabase {
    executors: Arc<Mutex<Vec<Executor>>>,
}

#[cfg(test)]
impl InMemoryExecutorDatabase {
    fn get_instance(&self) -> Vec<Executor> {
        self.executors
            .lock()
            .expect("Can not get Executors from InMemoryDatabase.")
            .clone()
    }
}

#[cfg(test)]
impl ExecutorRepository for InMemoryExecutorDatabase {
    fn push(&self, executor: Arc<Executor>, _: &'static str) -> ResultFuture<Arc<Executor>> {
        {
            let executor = executor.clone();
            let executors = self.get_instance();
            match executors.iter().find(|e| **e == (&*executor).clone()) {
                Some(_) => {
                    return future::ready(Err(ErrorKind::AlreadyExistsExecutor.into())).boxed()
                }
                None => { /* do nothing */ }
            }
        }
        {
            let mut executors = self.get_instance();
            executors.push((&*executor).clone());
            future::ok(executor).boxed()
        }
    }

    fn executor_of_name(&self, name: ExecutorName, _: &'static str) -> ResultFuture<Arc<Executor>> {
        let executors = self.get_instance();
        let mut executors_iter = executors.iter().filter(|e| &e.name == &name);
        let first_executor = executors_iter.next();
        if executors_iter.next().is_some() {
            unreachable!(
                "InMemoryDatabase should have less executor than one, but second of executor is found"
            );
        }
        match first_executor {
            None => future::ready(Err(ErrorKind::NotFoundExecutor.into())).boxed(),
            Some(e) => future::ok(Arc::new(e.clone())).boxed(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_executor() {
        let in_memory_database = InMemoryExecutorDatabase {
            executors: Arc::new(Mutex::new(Vec::new())),
        };

        let new_executor = Arc::new(Executor::new("test_executor".to_string().into()));
        let push_future = in_memory_database.push(new_executor.clone(), "");

        assert_eq!(
            futures::executor::ThreadPool::new()
                .expect("Future executor can't be initialized.")
                .run(push_future)
                .expect("Should return Executor"),
            new_executor
        )
    }

    #[test]
    fn get_executor() {
        let target_executor = Arc::new(Executor::new("test_executor".to_string().into()));
        let in_memory_database = InMemoryExecutorDatabase {
            executors: Arc::new(Mutex::new(vec![(&*target_executor).clone()])),
        };

        let get_future =
            in_memory_database.executor_of_name("test_executor".to_string().into(), "");

        assert_eq!(
            futures::executor::ThreadPool::new()
                .expect("Future executor can't be initialized.")
                .run(get_future)
                .expect("Should get a Executor."),
            target_executor
        )
    }

    #[test]
    fn not_found_executor() {
        let in_memory_database = InMemoryExecutorDatabase {
            executors: Arc::new(Mutex::new(Vec::new())),
        };

        let get_future =
            in_memory_database.executor_of_name("test_executor".to_string().into(), "");

        assert_eq!(
            futures::executor::ThreadPool::new()
                .expect("Future executor can't be initialized.")
                .run(get_future)
                .expect_err("Should be disable to get any Executor")
                .kind(),
            &ErrorKind::NotFoundExecutor
        )
    }

    #[test]
    fn already_exists_executor() {
        let target_executor = Arc::new(Executor::new("test_executor".to_string().into()));
        let in_memory_database = InMemoryExecutorDatabase {
            executors: Arc::new(Mutex::new(vec![(&*target_executor).clone()])),
        };

        let get_future = in_memory_database.push(target_executor, "");

        assert_eq!(
            futures::executor::ThreadPool::new()
                .expect("Future executor can't be initialized.")
                .run(get_future)
                .expect_err("Should be disable to push second same Executor")
                .kind(),
            &ErrorKind::AlreadyExistsExecutor
        )
    }
}
