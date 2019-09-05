use crate::resource::*;
use std::sync::Arc;

pub trait SSHExecutorRepository {
    fn push(
        &'static self,
        ssh_executor: Arc<SSHExecutor>,
    ) -> Box<dyn Future<Item = Arc<SSHExecutor>, Error = Error>>;
}

pub trait SSHExecutorRepositoryComponent {
    type Repository: SSHExecutorRepository;

    fn ssh_executor_repository(&self) -> &Self::Repository;
}
