use crate::log::*;

pub struct LogEvent<'a> {
    pub log_id: &'a LogID,
}
