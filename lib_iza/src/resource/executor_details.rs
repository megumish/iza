use crate::resource::*;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ExecutorDetails {
    details: HashMap<&'static str, String>,
}
