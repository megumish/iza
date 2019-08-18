#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "io error")]
    IOError,
    #[fail(display = "yaml serialize or deserialize error")]
    YamlParseError,
    #[fail(display = "already exist package")]
    AlreadyExistPackage,
    #[fail(display = "not found package")]
    NotFoundPackage,
}

pub type Result<T> = std::result::Result<T, Error>;
pub type RetFuture<T> = std::pin::Pin<Box<dyn futures::future::Future<Output = Result<T>> + Send>>;

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
