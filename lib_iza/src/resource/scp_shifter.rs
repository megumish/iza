use crate::resource::*;

pub struct SCPShifter {
    id: ShifterID,
    destination: SCPDestination,
}

impl SCPShifter {
    pub fn try_new((id, menu): (&ShifterID, &ShifterMenu)) -> Result<Self, Error> {
        let (destination,) = menu.parse_scp_menu()?;

        let id = (*id).clone();
        Ok(Self { id, destination })
    }
}
