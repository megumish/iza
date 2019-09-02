use chrono::prelude::*;

pub struct IssuedAt {
    at_date_time: chrono::DateTime<Local>,
}

impl IssuedAt {
    pub fn new() -> Self {
        let at_date_time = Local::now();

        Self { at_date_time }
    }
}
