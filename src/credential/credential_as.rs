use crate::credential::*;
use std::collections::HashMap;
use std::sync::Arc;

pub trait CredentialAs: Clone {
    fn id_of_credential(&self) -> String;

    fn kind_of_credential(&self) -> String;

    fn hash_map(&self) -> HashMap<String, String>;

    fn arc_hash_map(&self) -> Arc<HashMap<String, String>>;

    fn try_from_hash_map(h: HashMap<String, String>, id: String) -> Result<Self>;

    fn try_arc_from_arc_hash_map(h: Arc<HashMap<String, String>>, id: String) -> Result<Arc<Self>>;
}
