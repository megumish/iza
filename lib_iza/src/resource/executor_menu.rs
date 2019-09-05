use crate::resource::*;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ExecutorMenu {
    menu: HashMap<&'static str, String>,
}

impl ExecutorMenu {
    pub fn parse_ssh_menu(&self) -> Result<(SSHUser, SSHHost), Error> {
        let mut not_enough_menu = Vec::new();

        let user;
        match self.menu.get("user") {
            Some(u) => user = Some(u),
            None => {
                user = None;
                not_enough_menu.push("user");
            }
        }

        let host;
        match self.menu.get("host") {
            Some(h) => host = Some(h),
            None => {
                host = None;
                not_enough_menu.push("host");
            }
        }

        if !not_enough_menu.is_empty() {
            Err(Error::NotEnoughExecutorMenu(not_enough_menu))
        } else {
            Ok((
                SSHUser::new(user.unwrap().to_owned()),
                SSHHost::new(host.unwrap().to_owned()),
            ))
        }
    }
}
