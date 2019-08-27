use crate::wording::*;
use futures::prelude::*;

trait WordingDaemonApp: HasRunWordingDaemonService {
    fn run_wording_daemon(&'static self) -> ResultFuture<()> {
        self.run_wording_daemon_service().run().boxed()
    }
}
