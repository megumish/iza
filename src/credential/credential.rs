use crate::credential::*;
use std::convert::TryInto;

pub struct Credential {
    id: CredentialID,
    kind: CredentialKind,
}

impl Credential {
    pub fn restore(id: String, kind: String) -> Result<Self> {
        let id: CredentialID = id.into();
        let kind: CredentialKind = kind.try_into()?;
        Ok(Self { id, kind })
    }
}
