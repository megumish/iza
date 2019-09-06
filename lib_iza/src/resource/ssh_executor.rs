use crate::resource::*;

impl Executor for SSHExecutor {
    fn new_executor<ED>(ed: ED) -> Result<Self, Error>
    where
        ED: Into<ExecutorDetails>,
        Self: Sized,
    {
        let ed = ed.into();

        let user;
        let host;
        matches_executor_details!(ed, user, host);

        let id = ExecutorID::new();
        Ok(Self { id, user, host })
    }
}
