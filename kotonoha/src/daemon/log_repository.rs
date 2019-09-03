use crate::daemon::*;
use futures::prelude::*;
use std::pin::Pin;
use std::sync::Arc;

pub trait LogRepository {
    fn log_of_id<LI>(&self, log_id: LI) -> Pin<Box<dyn Future<Output = Result<Arc<Log>>> + Send>>
    where
        LI: Into<LogID>;
}
