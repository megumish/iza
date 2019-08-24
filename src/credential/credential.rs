use crate::credential::*;
use crate::dot_iza::*;
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Arc;

#[derive(Clone)]
pub struct Credential {
    id: CredentialID,
    kind: CredentialKind,
}

impl Credential {
    pub fn try_new(info: Arc<HashMap<String, String>>, kind: String) -> Result<Self> {
        let id: CredentialID = CredentialID::try_new(info)?;
        let kind: CredentialKind = kind.try_into()?;

        Ok(Self { id, kind })
    }

    pub fn try_new_arc(info: Arc<HashMap<String, String>>, kind: String) -> Result<Arc<Self>> {
        Ok(Arc::new(Self::try_new(info, kind)?))
    }

    pub fn try_restore(id: String, kind: String) -> Result<Self> {
        let id: CredentialID = id.into();
        let kind: CredentialKind = kind.try_into()?;

        Ok(Self { id, kind })
    }

    pub fn try_restore_arc(id: String, kind: String) -> Result<Arc<Self>> {
        Ok(Arc::new(Self::try_restore(id, kind)?))
    }

    pub fn id_of_credential(&self) -> CredentialID {
        self.id.clone()
    }

    pub fn kind_of_credential(&self) -> CredentialKind {
        self.kind.clone()
    }
}

impl Module for Credential {}
