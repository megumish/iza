use crate::credential::*;
use crate::dot_iza::*;
use crate::ssh_connection::*;
use futures::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct SSHConnection {
    id: SSHConnectionID,
    user: User,
    host: Host,
}

impl SSHConnection {
    pub fn restore(id: String, user: String, host: String) -> Self {
        let id = id.into();
        let user = User::new(user);
        let host = Host::new(host);
        Self { id, user, host }
    }

    pub fn id_of_ssh_connection(&self) -> SSHConnectionID {
        self.id.clone()
    }

    pub fn user_of_ssh_connection(&self) -> User {
        self.user.clone()
    }

    pub fn host_of_ssh_connection(&self) -> Host {
        self.host.clone()
    }
}

impl CredentialAs for SSHConnection {
    fn id_of_credential(&self) -> String {
        self.id.to_string()
    }

    fn kind_of_credential(&self) -> String {
        "SSHConnection".to_owned()
    }

    fn hash_map(&self) -> HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert("user".to_owned(), self.user.to_string());
        h.insert("host".to_owned(), self.host.to_string());
        h
    }

    fn arc_hash_map(&self) -> Arc<HashMap<String, String>> {
        Arc::new(self.hash_map())
    }

    fn try_from_hash_map(
        info: HashMap<String, String>,
        id: String,
    ) -> crate::credential::Result<Self> {
        let mut not_enough_info = Vec::new();

        let user;
        match info.get("user") {
            Some(n) => user = Some(n),
            None => {
                user = None;
                not_enough_info.push("user".to_owned());
            }
        }
        let host;
        match info.get("host") {
            Some(h) => host = Some(h),
            None => {
                host = None;
                not_enough_info.push("host".to_owned());
            }
        }

        if !not_enough_info.is_empty() {
            Err(crate::credential::ErrorKind::NotEnoughInfo(
                crate::credential::NotEnoughInfo::new(not_enough_info),
            ))?;
        }

        Ok(Self::restore(
            id,
            user.unwrap().to_owned(),
            host.unwrap().to_owned(),
        ))
    }

    fn try_arc_from_arc_hash_map(
        info: Arc<HashMap<String, String>>,
        id: String,
    ) -> crate::credential::Result<Arc<Self>> {
        let info = (&*info).clone();
        Ok(Arc::new(Self::try_from_hash_map(info, id)?))
    }
}

impl Module for SSHConnection {}
