#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "dot iza error")]
    DotIza,
    #[fail(display = "package error")]
    Package,
    #[fail(display = "already exists execution wording")]
    AlreadyExistsExecutionWording,
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

impl From<crate::package::Error> for Error {
    fn from(error: crate::package::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::Package),
        }
    }
}
