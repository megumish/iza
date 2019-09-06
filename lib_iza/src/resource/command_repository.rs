use crate::resource::*;

pub trait CommandRepository {
    fn push(
        &'static self,
        command: Arc<Command>,
    ) -> Box<dyn Future<Item = Arc<Command>, Error = Error>>;

    fn commands_of_ids(
        &'static self,
        command_ids: Vec<CID>,
    ) -> Box<dyn Future<Item = Vec<Arc<Command>>, Error = Error>>;
}

pub trait CommandRepositoryComponent {
    type Repository: CommandRepository;

    fn command_repository(&self) -> &Self::Repository;
}
