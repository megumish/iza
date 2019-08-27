use crate::package::*;
use futures::prelude::*;
use std::sync::Arc;

pub trait ExecutorApp: HasExecutorRepository + Sync {
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
