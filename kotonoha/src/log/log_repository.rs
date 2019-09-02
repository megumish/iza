use crate::log::*;
use std::sync::Arc;

pub trait LogRepository {
    fn push(&self, log: Arc<Log>) -> ResultFuture<Arc<Log>>;
}

pub trait LogRepositoryComponent {
    type Repository: LogRepository;

    fn log_repository(&self) -> &Self::Repository;
}
