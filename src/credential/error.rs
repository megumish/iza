use std::fmt;

pub enum Error {
    InvalidCredentialKind(String),
    NotEnoughInfo(Vec<String>),
    OtherAppError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidCredentialKind(k) => write!(f, "InvalidCredentialKind: {}", k),
            NotEnoughInfo(v) => {
                let mut full_text = "NotEnoughInfo: ".to_owned();
                for s in v {
                    full_text = format!("{}\n{}", full_text, s);
                }
                write!(f, "{}", full_text)
            }
            OtherAppError => write!(f, "OtherAppError"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
