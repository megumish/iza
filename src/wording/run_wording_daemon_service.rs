use crate::wording::*;

pub trait RunWordingDaemonService {
    fn run(&'static self) -> ResultFuture<()>;
}

pub trait HasRunWordingDaemonService {
    type Service: RunWordingDaemonService;

    fn run_wording_daemon_service(&self) -> &Self::Service;
}
