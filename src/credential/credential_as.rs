use crate::credential::*;

pub trait CredentialAs {
    fn get_credential_id(&self) -> String;

    fn get_credential_kind(&self) -> String;
}
