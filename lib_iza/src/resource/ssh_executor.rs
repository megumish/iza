use crate::resource::*;

pub struct SSHExecutor {
    id: ExecutorID,
    user: SSHUser,
    host: SSHHost,
}
