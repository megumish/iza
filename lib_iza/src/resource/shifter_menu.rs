use crate::resource::*;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ShifterMenu {
    menu: HashMap<&'static str, String>,
}

impl ShifterMenu {
    pub fn parse_scp_menu(&self) -> Result<(SCPDestination,), Error> {
        let mut not_enough_menu = Vec::new();

        let destination;
        match self.menu.get("destination") {
            Some(s) => destination = Some(s),
            None => {
                destination = None;
                not_enough_menu.push("destination");
            }
        }

        if !not_enough_menu.is_empty() {
            Err(Error::NotEnoughShifterMenu(not_enough_menu))
        } else {
            Ok((SCPDestination::new(destination.unwrap().to_owned()),))
        }
    }
}
