pub struct LogID {
    id_string: String,
}

impl<TS> From<TS> for LogID
where
    TS: ToString,
{
    fn from(id: TS) -> Self {
        let id_string = id.to_string();
        Self { id_string }
    }
}
