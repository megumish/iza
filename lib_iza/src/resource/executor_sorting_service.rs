use crate::resource::*;

pub trait ExecutorSortingService: SSHExecutorRepositoryComponent {
    fn push(
        &'static self,
        executor: Arc<Executor>,
    ) -> Box<dyn Future<Item = Arc<Executor>, Error = Error>> {
        match executor.kind_of_executor() {
            ExecutorKind::SSHExecutor => Box::new(
                future::result(
                    SSHExecutor::try_new(executor.summary_of_executor()).map(|s| Arc::new(s)),
                )
                .and_then(move |s| self.ssh_executor_repository().push(s))
                .and_then(move |_| future::ok(executor.clone())),
            ),
        }
    }
}

pub trait ExecutorSortingServiceComponent {
    type Service: ExecutorSortingService;

    fn executor_sorting_service(&self) -> &Self::Service;
}
