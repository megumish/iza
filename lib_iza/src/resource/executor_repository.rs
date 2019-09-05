use crate::resource::*;
use std::sync::Arc;

pub trait ExecutorRepository {
    fn push(
        &'static self,
        executor: Arc<Executor>,
    ) -> Box<dyn Future<Item = Arc<Executor>, Error = Error>>;
}

pub trait ExecutorRepositoryComponent {
    type Repository: ExecutorRepository;

    fn executor_repository(&self) -> &Self::Repository;
}
