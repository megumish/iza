use std::fmt;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "io error")]
    IOError,
    #[fail(display = "yaml serialize or deserialize error")]
    YamlParseError,
    #[fail(display = "already exist credential")]
    AlreadyExistCredential,
    #[fail(display = "not found credential")]
    NotFoundCredential,
    #[fail(display = "invalid credential kind: {}", _0)]
    InvalidCredentialKind(String),
    #[fail(display = "not enough info: {}", _0)]
    NotEnoughInfo(NotEnoughInfo),
    #[fail(display = "other app error")]
    OtherAppError,
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
