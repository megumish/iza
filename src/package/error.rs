pub enum Error {
    IOError,
    TomlDeserializeError,
    TomlSerializeError,
    AlreadyExistPackage,
    NotFoundPackage,
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        eprintln!("{}", error);
        Error::IOError
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        eprintln!("{}", error);
        Error::TomlDeserializeError
    }
}

impl From<toml::ser::Error> for Error {
    fn from(error: toml::ser::Error) -> Self {
        eprintln!("{}", error);
        Error::TomlSerializeError
    }
}
