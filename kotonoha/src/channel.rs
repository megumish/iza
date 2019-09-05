//! Use log server through Channel.
use crate::log::*;
use futures::{future, prelude::*, sync::mpsc};
use std::sync::Arc;

/// Channel is a instance as interface for Library User.
pub trait Channel {
    /// LogSender send log to this Channel
    type LogSender: LogSender;

    /// get LogSender
    fn get_sender(&self) -> &Self::LogSender;

    /// run channel
    fn run(self) -> Box<dyn Future<Item = (), Error = ()>>;
}

/// LogSender send log to a Channel
pub trait LogSender {
    /// send Log
    fn send<L>(&self, log: L) -> Box<dyn Future<Item = Arc<L>, Error = Error>>
    where
        L: Log + 'static;

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
pub struct StdoutChannel {
    log_sender: mpsc::Sender<Message>,
    log_receiver: mpsc::Receiver<Message>,
}

impl StdoutChannel {
    /// constructor
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(5);
        Self {
            log_sender: sender,
            log_receiver: receiver,
        }
    }
}

impl Channel for StdoutChannel {
    type LogSender = mpsc::Sender<Message>;

    fn run(self) -> Box<dyn Future<Item = (), Error = ()>> {
        Box::new(
            self.log_receiver
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
        )
    }

    fn get_sender(&self) -> &Self::LogSender {
        &self.log_sender
    }
}

impl LogSender for mpsc::Sender<Message> {
    fn send<L>(&self, log: L) -> Box<dyn Future<Item = Arc<L>, Error = Error>>
    where
        L: Log + 'static,
    {
        let log = Arc::new(log);
        let sender = self.clone();
        Box::new(
            sender
                .send(Message::Sending(log.clone()))
                .map_err(Into::into)
                .and_then(|_| future::ok(log)),
        )
    }

    fn finish(&self) -> Box<dyn Future<Item = (), Error = Error>> {
        let sender = self.clone();
        Box::new(
            sender
                .send(Message::Finish)
                .map_err(Into::into)
                .and_then(|_| future::ok(())),
        )
    }
}
