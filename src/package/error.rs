#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "io error")]
    IO,
    #[fail(display = "yaml serialize or deserialize error")]
    YamlSerializeOrDeserialize,
    #[fail(display = "dot iza error")]
    DotIza,
}

pub type Result<T> = std::result::Result<T, Error>;
pub type ResultFuture<T> =
    std::pin::Pin<Box<dyn futures::Future<Output = std::result::Result<T, Error>> + Send>>;

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

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::IO),
        }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::YamlSerializeOrDeserialize),
        }
    }
}

impl From<crate::dot_iza::Error> for Error {
    fn from(error: crate::dot_iza::Error) -> Self {
        Error {
            inner: error.context(ErrorKind::DotIza),
        }
    }
}
