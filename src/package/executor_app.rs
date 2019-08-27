use crate::package::*;
use futures::prelude::*;
use std::sync::Arc;

pub trait ExecutorApp: HasExecutorRepository + Sync {
    fn new_executor<N>(
        &'static self,
        executor_name: N,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<Executor>>
    where
        N: Into<ExecutorName>,
    {
        let executor = Arc::new(Executor::new(executor_name.into()));
        self.executor_repository()
            .push(executor, working_directory)
            .boxed()
    }

    fn execute(
        &'static self,
        executor_name: String,
        working_directory: &'static str,
    ) -> ResultFuture<Vec<Arc<Execution>>> {
        self.executor_repository()
            .executor_of_name(executor_name.into(), working_directory)
            .and_then(|e| e.execute())
            .boxed()
    }
}

pub trait HasExecutorApp {
    type App: ExecutorApp;

    fn executor_app(&self) -> &Self::App;
}

impl<T> ExecutorApp for T where T: HasExecutorRepository + Sync {}
