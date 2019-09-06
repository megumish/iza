use crate::resource::*;
use std::process::Command as ShellCommand;

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

    fn new_execution_of_command_strings(
        &self,
        command_strings: &CommandStrings,
    ) -> Arc<Execution<Box<dyn FnOnce()>>> {
        let user_str = self.user.to_string();
        let host_str = self.host.to_string();
        let command_line_string = command_strings.to_command_line_string();
        Arc::new(Execution::new(Box::new(move || {
            let _ = ShellCommand::new("ssh")
                .arg(format!("{}@{}", user_str.clone(), host_str.clone()))
                .arg(command_line_string)
                .output();
        })))
    }
}

impl CommandExecutor for SSHExecutor {}
