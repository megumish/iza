//! Use log server through Channel.
use crate::log::*;
use futures::{future, prelude::*, sync::mpsc};
use std::sync::Arc;

/// Channel is a instance as interface for Library User.
pub trait Channel {
    /// LogSender is received logs and then send Channel listner.
    type LogSender: LogSender;

    /// get LogSender of This Channel
    fn get_log_sender(&self) -> Self::LogSender;
}

/// LogSender is received logs and then send Channel listner.
pub trait LogSender: Clone {
    /// send Log
    fn send<L>(&self, log: L) -> Box<dyn Future<Item = Arc<L>, Error = Error>>
    where
        L: Log + 'static;
}

/// Error type about Channel
pub enum Error {
    /// Failed to send
    FailedToSend,
}

impl From<mpsc::SendError<Arc<dyn Log>>> for Error {
    fn from(error: mpsc::SendError<Arc<dyn Log>>) -> Self {
        panic!("LogSendError: {:#?}", error)
    }
}

/// This Channel output to stdout
pub struct StdoutChannel {
    log_sender: StdoutLogSender,
}

/// LogSender of StdoutChannel;
#[derive(Clone)]
pub struct StdoutLogSender {
    inner: mpsc::Sender<Arc<dyn Log>>,
}

impl StdoutChannel {
    /// constructor
    pub fn new() -> impl Future<Item = StdoutLogSender, Error = ()> {
        let (sender, mut receiver) = mpsc::channel(5);
        future::ok(Self {
            log_sender: StdoutLogSender::new(sender),
        })
        .join(future::ok(receiver.close()))
        .and_then(|(c, _)| future::ok(c.get_log_sender()))
    }
}

impl StdoutLogSender {
    /// constructor
    pub fn new(inner: mpsc::Sender<Arc<dyn Log>>) -> Self {
        Self { inner }
    }
}

impl Channel for StdoutChannel {
    type LogSender = StdoutLogSender;

    fn get_log_sender(&self) -> Self::LogSender {
        self.log_sender.clone()
    }
}

impl LogSender for StdoutLogSender {
    fn send<L>(&self, log: L) -> Box<dyn Future<Item = Arc<L>, Error = Error>>
    where
        L: Log + 'static,
    {
        let log = Arc::new(log);
        let sender = self.inner.clone();
        Box::new(
            sender
                .send(log.clone())
                .map_err(Into::into)
                .and_then(|_| future::ok(log)),
        )
    }
}
