use crate::resource::*;

pub struct CommandStringsRaw {
    raw_string: String,
}

impl CommandStringsRaw {
    pub fn parse(self) -> CommandStrings {
        CommandStrings::new(self.raw_string.split_ascii_whitespace())
    }
}
