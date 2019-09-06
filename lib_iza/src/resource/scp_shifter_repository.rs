use crate::resource::*;
use std::sync::Arc;

pub trait SCPShifterRepository {
    fn push(
        &'static self,
        scp_shifter: Arc<SCPShifter>,
    ) -> Box<dyn Future<Item = Arc<SCPShifter>, Error = Error>>;
}

pub trait SCPShifterRepositoryComponent {
    type Repository: SCPShifterRepository;

    fn scp_shifter_repository(&self) -> &Self::Repository;
}
