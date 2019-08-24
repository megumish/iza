#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "invalid credential kind: {}", _0)]
    InvalidCredentialKind(String),
    #[fail(display = "not enough info: {}", _0)]
    NotEnoughInfo(NotEnoughInfo),
    #[fail(display = "dot iza system error")]
    DotIza,
    #[fail(display = "ssh connection error")]
    SSHConnection,
    #[fail(display = "json serialize or deserialize error")]
    JsonSerializeOrDeserialize,
}

impl fmt::Display for NotEnoughInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut full_text = "NotEnoughInfo: ".to_owned();
        for s in &self.info {
            full_text = format!("{}\n{}", full_text, s);
        }
        write!(f, "{}", full_text)
    }
}

#[derive(Debug)]
pub struct NotEnoughInfo {
    info: Vec<String>,
}

impl NotEnoughInfo {
    pub fn new(info: Vec<String>) -> Self {
        Self { info }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ResultFuture<T> =
    std::pin::Pin<Box<dyn futures::future::Future<Output = Result<T>> + Send>>;

/* ----------- failure boilerplate ----------- */

use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<crate::dot_iza::Error> for Error {
    fn from(error: crate::dot_iza::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::DotIza),
        }
    }
}

impl From<crate::ssh_connection::Error> for Error {
    fn from(error: crate::ssh_connection::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::SSHConnection),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::JsonSerializeOrDeserialize),
        }
    }
}
