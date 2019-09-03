use crate::daemon::*;
use crate::log::*;
use futures::{io::*, prelude::*};
use std::marker::Unpin;

use crate::daemon::{Error, Result};

pub struct Daemon<LR>
where
    LR: LogRepository,
{
    log_repository: LR,
}

impl<LR> Daemon<LR>
where
    LR: LogRepository,
{
    fn run_future<IS, O>(self, events: IS, sink: O) -> impl Future
    where
        IS: Stream<Item = LogEvent<'static>>,
        O: Sink<String, Error = Error> + Unpin,
    {
        events
            .then(move |e| self.log_repository.log_of_id(e.log_id))
            .try_for_each(move |l| sink.send("NYAN".to_string()))
    }
}
