use crate::resource::*;

pub trait CommandRepository {
    fn push(
        &'static self,
        command: Arc<Command>,
    ) -> Box<dyn Future<Item = Arc<Command>, Error = Error>>;
}

pub trait CommandRepositoryComponent {
    type Repository: CommandRepository;

    fn command_repository(&self) -> &Self::Repository;
}
