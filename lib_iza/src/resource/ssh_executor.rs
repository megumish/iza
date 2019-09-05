use crate::resource::*;

pub struct SSHExecutor {
    id: ExecutorID,
    user: SSHUser,
    host: SSHHost,
}

impl SSHExecutor {
    pub fn try_new((id, menu): (&ExecutorID, &ExecutorMenu)) -> Result<Self, Error> {
        let (user, host) = menu.parse_ssh_menu()?;

        let id = (*id).clone();
        Ok(Self { id, user, host })
    }
}
