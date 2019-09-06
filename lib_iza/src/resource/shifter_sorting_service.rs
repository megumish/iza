use crate::resource::*;

pub trait ShifterSortingService: SCPShifterRepositoryComponent {
    fn push(
        &'static self,
        shifter: Arc<Shifter>,
    ) -> Box<dyn Future<Item = Arc<Shifter>, Error = Error>> {
        match shifter.kind_of_shifter() {
            ShifterKind::SCPShifter => Box::new(
                future::result(
                    SCPShifter::try_new(shifter.summary_of_shifter()).map(|l| Arc::new(l)),
                )
                .and_then(move |l| self.scp_shifter_repository().push(l))
                .and_then(move |_| future::ok(shifter.clone())),
            ),
        }
    }
}

pub trait ShifterSortingServiceComponent {
    type Service: ShifterSortingService;

    fn shifter_sorting_service(&self) -> &Self::Service;
}
