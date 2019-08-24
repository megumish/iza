use crate::credential::*;
use crate::dot_iza::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, PartialOrd, Clone, Hash, Eq, Ord, Serialize, Deserialize)]
pub struct YamlCredential {
    id: String,
    kind: String,
}

impl YamlModule<Credential> for YamlCredential {
    fn new_yaml_module(credential: Arc<Credential>) -> Self {
        let credential: Credential = (&*credential).clone();
        let id = credential.id_of_credential().to_string();
        let kind = credential.kind_of_credential().to_string();
        Self { id, kind }
    }

    fn restore(&self) -> Credential {
        Credential::try_restore(self.id.clone(), self.kind.clone()).expect(&format!(
            "不正なクレデンシャルを復元しようとしました。 {:#?}",
            self,
        ))
    }
}
