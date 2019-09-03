use crate::log::*;
use futures::prelude::*;
use std::pin::Pin;
use std::sync::Arc;

pub trait LogRepository {
    fn push(&self, log: Arc<Log>) -> Pin<Box<dyn Future<Output = Result<Arc<Log>>> + Send>>;
}

pub trait LogRepositoryComponent {
    type Repository: LogRepository;

    fn log_repository(&self) -> &Self::Repository;
}
