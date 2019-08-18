use crate::credential::*;
use crate::dot_iza::*;
use std::convert::TryInto;

#[derive(Clone)]
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

    pub fn id_of_credential(&self) -> CredentialID {
        self.id.clone()
    }

    pub fn kind_of_credential(&self) -> CredentialKind {
        self.kind.clone()
    }
}

impl Module for Credential {}
