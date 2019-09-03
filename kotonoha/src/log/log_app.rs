use crate::log::*;
use futures::prelude::*;
use std::pin::Pin;
use std::sync::Arc;

pub trait LogApp: LogRepositoryComponent + Sync {
    fn push_log<F, LI>(
        &'static self,
        log_info: LI,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<Log>>> + Send>>
    where
        LI: Into<LogInfo>,
    {
        future::ok(Arc::new(Log::new(log_info)))
            .and_then(move |l| self.log_repository().push(l))
            .boxed()
    }
}
