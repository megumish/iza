use crate::wording::*;
use std::sync::Arc;

pub trait ExecutionWordingRepository {
    fn push(&'static self, wording: Arc<ExecutionWording>) -> ResultFuture<Arc<ExecutionWording>>;
}

pub trait HasExecutionWordingRepository {
    type Repository: ExecutionWordingRepository;

    fn execution_wording_repository(&self) -> &Self::Repository;
}
