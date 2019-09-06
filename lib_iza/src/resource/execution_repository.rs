pub trait ExecutionRepository {}

pub trait ExecutionRepositoryComponent {
    type Repository: ExecutionRepository;

    fn execution_repository(&self) -> &Self::Repository;
}
