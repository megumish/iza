use crate::wording::*;
use std::sync::Arc;

pub trait ExecutorWordingRepository {
    fn push(&self, wording: Arc<ExecutorWording>) -> ResultFuture<Arc<ExecutorWording>>;
}

pub trait HasExecutorWordingRepository {
    type Repository: ExecutorWordingRepository;

    fn executor_wording_repository(&self) -> &Self::Repository;
}
