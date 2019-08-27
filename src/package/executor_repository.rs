use crate::package::*;
use std::sync::Arc;

pub trait ExecutorRepository {
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
