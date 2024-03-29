use crate::ssh_connection::*;
#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "dot iza error")]
    DotIza,
    #[fail(display = "credential error")]
    Credential,
    #[fail(display = "not found this ssh connection of id: {:#?}", _0)]
    NotFoundSSHConnection(SSHConnectionID),
    #[fail(display = "invalid local path")]
    InvalidLocalPath,
    #[fail(display = "io error")]
    IO,
    #[fail(display = "string error")]
    Utf8,
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

impl From<crate::credential::Error> for Error {
    fn from(error: crate::credential::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::Credential),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::IO),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error {
            inner: error.context(ErrorKind::Utf8),
        }
    }
}
