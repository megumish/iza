use crate::log::*;

pub struct Log {
    id: LogID,
    info: LogInfo,
    issued_at: IssuedAt,
}

impl Log {
    pub fn new<LI>(log_info: LI) -> Self
    where
        LI: Into<LogInfo>,
    {
        let id = LogID::new();
        let info = log_info.into();
        let issued_at = IssuedAt::new();

        Self {
            id,
            info,
            issued_at,
        }
    }
}
