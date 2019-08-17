use crate::object::*;

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

    pub fn get_id(&self) -> ObjectInfoID {
        self.id.clone()
    }
}
