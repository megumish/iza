use crate::resource::*;
use std::sync::Arc;

pub trait ExecutorRepository {
    fn push<E>(&'static self, executor: Arc<E>) -> Box<dyn Future<Item = Arc<E>, Error = Error>>
    where
        E: Executor;

    fn executor_of_id<EID>(
        &'static self,
        executor_id: EID,
    ) -> Box<dyn Future<Item = Arc<dyn Executor>, Error = Error>>
    where
        EID: Into<ExecutorID>;

    fn command_executor_of_id<EID>(
        &self,
        executor_id: &EID,
    ) -> Box<dyn Future<Item = Arc<dyn CommandExecutor>, Error = Error>>
    where
        EID: Into<ExecutorID>;
}

pub trait ExecutorRepositoryComponent {
    type Repository: ExecutorRepository;

    fn executor_repository(&self) -> &Self::Repository;
}
