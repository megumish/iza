use crate::resource::*;
use std::sync::Arc;

pub trait ShifterRepository {
    fn push(
        &'static self,
        shifter: Arc<Shifter>,
    ) -> Box<dyn Future<Item = Arc<Shifter>, Error = Error>>;
}

pub trait ShifterRepositoryComponent {
    type Repository: ShifterRepository;

    fn shifter_repository(&self) -> &Self::Repository;
}
