use crate::resource::*;

pub trait ExecutionRepository {
    fn push(
        &'static self,
        execution: Arc<Execution<Box<dyn FnOnce()>>>,
    ) -> Box<dyn Future<Item = Arc<Execution<Box<dyn FnOnce()>>>, Error = Error>>;
}

pub trait ExecutionRepositoryComponent {
    type Repository: ExecutionRepository;

    fn execution_repository(&self) -> &Self::Repository;
}
