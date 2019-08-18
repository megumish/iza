#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid top path")]
    InvalidTopPath,
    #[fail(display = "io error")]
    IOError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        eprintln!("{}", error);
        Error::IOError
    }
}
