use crate::object::*;

#[derive(Clone)]
pub struct ObjectInfo {
    id: ObjectInfoID,
    local_path: LocalPath,
    remote_path: RemotePath,
    credential_id: CredentialID,
}

impl ObjectInfo {
    pub fn new(local_path: String, remote_path: String, credential_id: String) -> Self {
        let id = ObjectInfoID::new();
        let local_path: LocalPath = local_path.into();
        let remote_path: RemotePath = remote_path.into();
        let credential_id: CredentialID = credential_id.into();

        Self {
            id,
            local_path,
            remote_path,
            credential_id,
        }
    }

    pub fn restore(
        id: String,
        local_path: String,
        remote_path: String,
        credential_id: String,
    ) -> Self {
        let id = id.into();
        let local_path: LocalPath = local_path.into();
        let remote_path: RemotePath = remote_path.into();
        let credential_id: CredentialID = credential_id.into();

        Self {
            id,
            local_path,
            remote_path,
            credential_id,
        }
    }

    pub fn id_of_object_info(&self) -> ObjectInfoID {
        self.id.clone()
    }

    pub fn local_path_of_object_info(&self) -> LocalPath {
        self.local_path.clone()
    }

    pub fn remote_path_of_object_info(&self) -> RemotePath {
        self.remote_path.clone()
    }

    pub fn credential_id_of_object_info(&self) -> CredentialID {
        self.credential_id.clone()
    }
}
