//! Resource Domain represent interface of deployment resources
use futures::{future, prelude::*};
use std::sync::Arc;

/// Resource App is a interface for library user.
pub trait ResourceApp: ExecutorRepositoryComponent + CommandRepositoryComponent {
    /// new Command
    fn new_command<CS, EID>(
        &'static self,
        command_strings_raw: CS,
        executor_id: EID,
    ) -> Box<dyn Future<Item = Arc<Command>, Error = Error>>
    where
        CS: Into<CommandStringsRaw>,
        EID: Into<ExecutorID>,
    {
        Box::new(
            future::result(Command::try_new(command_strings_raw, executor_id).map(|c| Arc::new(c)))
                .and_then(move |c| self.command_repository().push(c)),
        )
    }

    /// new Executor
    fn new_executor<E, ED>(
        &'static self,
        executor_details: ED,
    ) -> Box<dyn Future<Item = Arc<E>, Error = Error>>
    where
        E: Executor + 'static,
        ED: Into<ExecutorDetails>,
    {
        Box::new(
            future::result(E::new_executor(executor_details).map(|e| Arc::new(e)))
                .and_then(move |e| self.executor_repository().push(e)),
        )
    }
}

/// Executor execute something for deployment
pub trait Executor {
    /// new Executor
    fn new_executor<E, ED>(executor_details: ED) -> Result<E, Error>
    where
        ED: Into<ExecutorDetails>,
        E: Executor;
}

/// A extension of Executor for command execution
pub trait CommandExecutor: Executor {}

/// Command is a unit of Execution
pub struct Command {
    id: CommandID,
    command_strings: CommandStrings,
    executor_id: ExecutorID,
}

impl Command {
    fn try_new<CS, EID>(command_strings_raw: CS, executor_id: EID) -> Result<Self, Error>
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
}

/// Resource Error
pub enum Error {
    /// Invalid Kind of Executor
    InvalidExecutorKind,
    /// Failed to generate new ExecutorID
    FailedNewExecutorID,
    /// Failed to generate new CommandID
    FailedNewCommandID,
}

mod command_id;
mod command_repository;
mod command_strings;
mod command_strings_raw;
mod executor_details;
mod executor_id;
mod executor_kind;
mod executor_repository;

pub(self) use self::command_id::*;
pub(self) use self::command_repository::*;
pub(self) use self::command_strings::*;
pub(self) use self::command_strings_raw::*;
pub(self) use self::executor_details::*;
pub(self) use self::executor_id::*;
pub(self) use self::executor_kind::*;
pub(self) use self::executor_repository::*;
