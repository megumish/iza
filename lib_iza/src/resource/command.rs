use crate::resource::*;

impl Command {
    pub(in crate::resource) fn try_new<CS, EID>(
        command_strings_raw: CS,
        executor_id: EID,
    ) -> Result<Self, Error>
    where
        CS: Into<CommandStringsRaw>,
        EID: Into<ExecutorID>,
    {
        let command_strings = command_strings_raw.into().parse();
        let executor_id = executor_id.into();

        let id = CommandID::try_new(&command_strings, &executor_id)?;

        Ok(Self {
            id,
            command_strings,
            executor_id,
        })
    }

    pub(in crate::resource) fn new_command_execution<S>(
        self,
        suite: &S,
    ) -> Box<dyn Future<Item = Arc<Execution<Box<dyn FnOnce()>>>, Error = Error>>
    where
        S: ExecutorRepositoryComponent,
    {
        Box::new(
            suite
                .executor_repository()
                .command_executor_of_id(&self.executor_id)
                .and_then(move |ce| {
                    future::ok(CommandExecutor::new_execution_of_command_strings(
                        &*ce,
                        &self.command_strings,
                    ))
                }),
        )
    }
}
