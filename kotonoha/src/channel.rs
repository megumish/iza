//! Use log server through Channel.
use crate::log::*;
use futures::{future, prelude::*, sync::mpsc};
use std::sync::Arc;

/// Channel is a instance as interface for Library User.
pub trait Channel {
    /// send Log
    fn send<L>(&self, log: L) -> Box<dyn Future<Item = Arc<L>, Error = Error>>
    where
        L: Log + 'static;

    /// run channel
    fn run(self) -> Box<dyn Future<Item = (), Error = ()>>;

    /// finish channel
    fn finish(&self) -> Box<dyn Future<Item = (), Error = Error>>;
}

/// Channel Message
pub enum Message {
    /// Message to send channel
    Sending(Arc<dyn Log>),
    /// Message to finish channel
    Finish,
}

/// Error type about Channel
pub enum Error {
    /// Failed to send
    FailedToSend,
}

impl From<mpsc::SendError<Message>> for Error {
    fn from(error: mpsc::SendError<Message>) -> Self {
        panic!("LogSendError: {:#?}", error)
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        unreachable!()
    }
}

/// This Channel output to stdout
#[derive(Clone)]
pub struct StdoutChannel {
    log_sender: mpsc::Sender<Message>,
    log_receiver: Arc<mpsc::Receiver<Message>>,
}

impl StdoutChannel {
    /// constructor
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(5);
        Self {
            log_sender: sender,
            log_receiver: Arc::new(receiver),
        }
    }
}

impl Channel for StdoutChannel {
    fn send<L>(&self, log: L) -> Box<dyn Future<Item = Arc<L>, Error = Error>>
    where
        L: Log + 'static,
    {
        let log = Arc::new(log);
        let sender = self.log_sender.clone();
        Box::new(
            sender
                .send(Message::Sending(log.clone()))
                .map_err(Into::into)
                .and_then(|_| future::ok(log)),
        )
    }

    fn run(self) -> Box<dyn Future<Item = (), Error = ()>> {
        loop {
            if let Ok(receiver) = Arc::try_unwrap(self.log_receiver.clone()) {
                break Box::new(
                    receiver
                        .take_while(|m| {
                            future::ok(match m {
                                Message::Finish => false,
                                _ => true,
                            })
                        })
                        .for_each(|m| {
                            future::ok(match m {
                                Message::Finish => {}
                                Message::Sending(l) => println!("{}", l.log_message()),
                            })
                        }),
                );
            }
        }
    }

    fn finish(&self) -> Box<dyn Future<Item = (), Error = Error>> {
        let sender = self.log_sender.clone();
        Box::new(
            sender
                .send(Message::Finish)
                .map_err(Into::into)
                .and_then(|_| future::ok(())),
        )
    }
}
