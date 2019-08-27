use crate::package::*;
use crate::wording::*;
use futures::prelude::*;
use std::sync::Arc;

use crate::wording::ResultFuture;

pub trait WithWordingApp:
    HasExecutorApp + HasExecutionWordingRepository + HasExecutorWordingRepository + Sync
{
    fn new_executor_with_wording(
        &'static self,
        executor_name: String,
        working_directory: &'static str,
    ) -> ResultFuture<Arc<ExecutorWording>> {
        self.executor_app()
            .new_executor(executor_name, working_directory)
            .map_err(Into::into)
            .and_then(|e| {
                future::ok(Arc::new(ExecutorWording {
                    executor_name: (&*e).name.to_string().into(),
                }))
            })
            .boxed()
    }

    fn deploy_with_wording(
        &'static self,
        executor_name: String,
        working_directory: &'static str,
    ) -> ResultFuture<Vec<Arc<ExecutionWording>>> {
        self.executor_app()
            .execute(executor_name, working_directory)
            .map_err(Into::into)
            .and_then(|es| {
                future::try_join_all(es.iter().map(|e| {
                    future::ok(Arc::new(ExecutionWording {
                        id: (&**e).id.to_string().into(),
                        execution_name: (&**e).name.to_string().into(),
                    }))
                }))
            })
            .and_then(move |ws| {
                future::try_join_all(
                    ws.iter()
                        .map(|w| self.execution_wording_repository().push(w.clone())),
                )
            })
            .boxed()
    }
}

trait HasWithWordingApp {
    type App: WithWordingApp;

    fn with_wording_app(&self) -> &Self::App;
}

impl<T> WithWordingApp for T where
    T: HasExecutorApp + HasExecutionWordingRepository + HasExecutorWordingRepository + Sync
{
}
