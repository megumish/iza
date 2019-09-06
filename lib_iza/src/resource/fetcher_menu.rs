use crate::resource::*;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct FetcherMenu {
    menu: HashMap<&'static str, String>,
}

impl FetcherMenu {
    pub fn parse_local_menu(&self) -> Result<(LocalSource,), Error> {
        let mut not_enough_menu = Vec::new();

        let source;
        match self.menu.get("source") {
            Some(s) => source = Some(s),
            None => {
                source = None;
                not_enough_menu.push("source");
            }
        }

        if !not_enough_menu.is_empty() {
            Err(Error::NotEnoughFetcherMenu(not_enough_menu))
        } else {
            Ok((LocalSource::new(source.unwrap().to_owned()),))
        }
    }
}
