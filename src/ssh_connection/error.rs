use std::fmt;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "io error")]
    IOError,
    #[fail(display = "yaml serialize or deserialize error")]
    YamlParseError,
    #[fail(display = "already exist ssh_connection")]
    AlreadyExistSSHConnection,
    #[fail(display = "not found ssh_connection")]
    NotFoundSSHConnection,
    #[fail(display = "invalid local path")]
    InvalidLocalPath,
    #[fail(display = "invalid output")]
    InvalidOutput,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        eprintln!("{}", error);
        Error::IOError
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        eprintln!("{}", error);
        Error::YamlParseError
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        eprintln!("{}", error);
        Error::InvalidOutput
    }
}
