use crate::resource::*;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct FetcherMenu {
    menu: HashMap<&'static str, String>,
}

impl FetcherMenu {
    pub fn parse_local_menu(&self) -> Result<(LocalSource,), Error> {
        let mut not_enough_menu = Vec::new();

        let local_source;
        match self.menu.get("local_source") {
            Some(s) => local_source = Some(s),
            None => {
                local_source = None;
                not_enough_menu.push("local_source");
            }
        }

        if !not_enough_menu.is_empty() {
            Err(Error::NotEnoughFetcherMenu(not_enough_menu))
        } else {
            Ok((LocalSource::new(local_source.unwrap().to_owned()),))
        }
    }
}
