#[derive(Serialize)]
pub struct CommandStrings {
    values: Vec<String>,
}

impl CommandStrings {
    pub fn new<'a, I>(values_iter: I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        Self {
            values: values_iter.map(ToString::to_string).collect(),
        }
    }
}
