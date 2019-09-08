//! Resource Domain represent interface of deployment resources
use futures::{future, prelude::*};
use std::sync::Arc;

/// Resource App is a interface for library user.
pub trait ResourceApp:
    ExecutorRepositoryComponent + CommandRepositoryComponent + ExecuteCommandServiceComponent
{
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

    /// execue Command
    fn execute_command<CID>(
        &'static self,
        command_ids: Vec<CID>,
    ) -> Box<dyn Future<Item = Vec<Arc<Execution<Box<dyn FnOnce()>>>>, Error = Error>>
    where
        CID: Into<CommandID>,
    {
        Box::new(
            self.execute_command_service()
                .execute_commands_of_ids(command_ids),
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

macro_rules! matches_executor_details {
    (
        $executor_details:ident,
        $($remain_detail_var:ident),*
    ) => {
        matches_executor_details!
        (
            =>
            ;$executor_details
            ;$($remain_detail_var)*
        )
    };

    (
        =>$($not_exist_detail:expr)+
        ;$executor_details:expr
        ;$detail_var:ident
        $($remain_detail_var:ident)*
    ) => {
        match ExecutorDetails::get(&$executor_details, stringify!($detail_var)) {
            None => {
                matches_executor_details!
                (
                    =>$($not_exist_detail)* stringify!($detail_var)
                    ;$executor_details
                    ;$($remain_detail_var)*
                )
            }
            Some(_x) => {
                matches_executor_details!
                (
                    =>$($not_exist_detail)*
                    ;$executor_details
                    ;$($remain_detail_var)*
                )
            }
        }
    };

    (
        =>$($not_exist_detail:expr)*
        ;$executor_details:expr
        ;$detail_var:ident
        $($remain_detail_var:ident)*
    ) => {
        match ExecutorDetails::get(&$executor_details, stringify!($detail_var)) {
            None => {
                matches_executor_details!
                (
                    =>$($not_exist_detail)* stringify!($detail_var)
                    ;$executor_details
                    ;$($remain_detail_var)*
                )
            }
            Some(x) => {
                $detail_var = x.to_owned().into();
                matches_executor_details!
                (
                    =>$($not_exist_detail)*
                    ;$executor_details
                    ;$($remain_detail_var)*
                )
            }
        }
    };

    (
        =>
        ;$_:expr
        ;
    ) => ((););

    (
        =>$($not_exist_detail:expr)+
        ;$_:expr
        ;
    ) => {
        return Err(crate::resource::Error::NotEnoughExecutorDetails(vec!(
            $($not_exist_detail),*
        )));
    };

}

/// Executor execute something for deployment
pub trait Executor {
    /// new Executor
    fn new_executor<ED>(executor_details: ED) -> Result<Self, Error>
    where
        ED: Into<ExecutorDetails>,
        Self: Sized;

    /// Executor generate new Execution of CommandStrings
    fn new_execution_of_command_strings(
        &self,
        command_strings: &CommandStrings,
    ) -> Arc<Execution<Box<dyn FnOnce()>>>;
}

/// A extension of Executor for command execution
pub trait CommandExecutor: Executor {
    /// Executor generate new Execution of CommandStrings
    fn new_execution_of_command_strings(
        &self,
        command_strings: &CommandStrings,
    ) -> Arc<Execution<Box<dyn FnOnce()>>>
where {
        Executor::new_execution_of_command_strings(self, command_strings)
    }
}

/// Kind of Executor for execution by SSH Connection
pub struct SSHExecutor {
    id: ExecutorID,
    user: SSHUser,
    host: SSHHost,
}

/// Command is a unit of Execution
#[derive(Clone)]
pub struct Command {
    id: CommandID,
    command_strings: CommandStrings,
    executor_id: ExecutorID,
}

/// Resource Error
pub enum Error {
    /// Invalid Kind of Executor
    InvalidExecutorKind,
    /// Failed to generate new ExecutorID
    FailedNewExecutorID,
    /// Failed to generate new CommandID
    FailedNewCommandID,
    /// Not Enough Executor Details
    NotEnoughExecutorDetails(Vec<&'static str>),
}

mod command;
mod command_id;
mod command_repository;
mod command_strings;
mod command_strings_raw;
mod execute_command_service;
mod execution;
mod execution_repository;
mod executor_details;
mod executor_id;
mod executor_repository;
mod ssh_executor;
mod ssh_host;
mod ssh_user;

pub(self) use self::command_id::*;
pub(self) use self::command_repository::*;
pub(self) use self::command_strings::*;
pub(self) use self::command_strings_raw::*;
pub(self) use self::execute_command_service::*;
pub(self) use self::execution::*;
pub(self) use self::execution_repository::*;
pub(self) use self::executor_details::*;
pub(self) use self::executor_id::*;
pub(self) use self::executor_repository::*;
pub(self) use self::ssh_host::*;
pub(self) use self::ssh_user::*;
