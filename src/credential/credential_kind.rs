use crate::credential::*;
use std::convert::TryFrom;

#[derive(Clone)]
pub enum CredentialKind {
    SSHConnection,
}

impl TryFrom<String> for CredentialKind {
    type Error = Error;

    fn try_from(kind_string: String) -> Result<Self> {
        match kind_string.as_str() {
            "SSHConnection" => Ok(CredentialKind::SSHConnection),
            x => Err(ErrorKind::InvalidCredentialKind(x.to_owned()).into()),
        }
    }
}

impl ToString for CredentialKind {
    fn to_string(&self) -> String {
        match self {
            CredentialKind::SSHConnection => "SSHConnection".to_owned(),
        }
    }
}
