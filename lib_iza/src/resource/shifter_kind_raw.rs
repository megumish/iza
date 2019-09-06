use crate::resource::*;

pub struct ShifterKindRaw {
    raw_string: &'static str,
}

impl ShifterKindRaw {
    pub fn try_parse(self) -> Result<ShifterKind, Error> {
        match self.raw_string {
            "SCPShifter" => Ok(ShifterKind::SCPShifter),
            _ => Err(Error::InvalidShifterKind),
        }
    }
}
