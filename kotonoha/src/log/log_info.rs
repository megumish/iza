use std::collections::HashMap;

pub struct LogInfo {
    log_info_hash_map: HashMap<&'static str, String>,
}

impl From<HashMap<&'static str, String>> for LogInfo {
    fn from(log_info_hash_map: HashMap<&'static str, String>) -> Self {
        Self { log_info_hash_map }
    }
}
