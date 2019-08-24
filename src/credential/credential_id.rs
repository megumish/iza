use crate::credential::*;
use blake2::{Blake2b, Digest};
use serde_json as json;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, PartialEq)]
pub struct CredentialID {
    id_string: String,
}

impl CredentialID {
    pub fn try_new(info: Arc<HashMap<String, String>>) -> Result<Self> {
        let info = &*info.clone();
        let serialized = json::to_vec(&info)?;
        let mut hasher = Blake2b::new();
        hasher.input(serialized);
        let id_string = hex::encode(hasher.result());

        Ok(Self { id_string })
    }
}

impl ToString for CredentialID {
    fn to_string(&self) -> String {
        self.id_string.to_owned()
    }
}

impl From<String> for CredentialID {
    fn from(id_string: String) -> Self {
        Self { id_string }
    }
}
