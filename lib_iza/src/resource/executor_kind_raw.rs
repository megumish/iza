use crate::resource::*;

pub struct ExecutorKindRaw {
    raw_string: &'static str,
}

impl ExecutorKindRaw {
    pub fn try_parse(self) -> Result<ExecutorKind, Error> {
        match self.raw_string {
            "SSHExecutor" => Ok(ExecutorKind::SSHExecutor),
            _ => Err(Error::InvalidExecutorKind),
        }
    }
}
