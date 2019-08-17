use crate::credential::*;
use std::convert::TryFrom;

pub enum CredentialKind {
    SSHConnection,
}

impl TryFrom<String> for CredentialKind {
    type Error = Error;

    fn try_from(kind_string: String) -> Result<Self> {
        match kind_string.as_str() {
            "SSHConnection" => Ok(CredentialKind::SSHConnection),
            x => Err(Error::InvalidCredentialKind(x.to_owned())),
        }
    }
}
