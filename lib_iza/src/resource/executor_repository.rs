use crate::resource::*;
use std::sync::Arc;

pub trait ExecutorRepository {
    fn push<E>(&'static self, executor: Arc<E>) -> Box<dyn Future<Item = Arc<E>, Error = Error>>
    where
        E: Executor;
}

pub trait ExecutorRepositoryComponent {
    type Repository: ExecutorRepository;

    fn executor_repository(&self) -> &Self::Repository;
}
