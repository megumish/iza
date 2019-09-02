use crate::log::*;
use futures::prelude::*;
use std::sync::Arc;

pub trait LogApp: LogRepositoryComponent + Sync {
    fn push_log<LI>(&'static self, log_info: LI) -> ResultFuture<Arc<Log>>
    where
        LI: Into<LogInfo>,
    {
        future::ok(Arc::new(Log::new(log_info)))
            .and_then(move |l| self.log_repository().push(l))
            .boxed()
    }
}
