//! Resource Domain represent interface of deployment resources
use futures::{future, prelude::*};
use std::sync::Arc;

/// Resource App is a interface for library user.
pub trait ResourceApp:
    ExecutorRepositoryComponent
    + FetcherRepositoryComponent
    + FetcherSortingServiceComponent
    + ShifterRepositoryComponent
    + ShifterSortingServiceComponent
    + CommandRepositoryComponent
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

    /// new Fetcher
    fn new_fetcher<FK, FM>(
        &'static self,
        fetcher_kind_raw: FK,
        fetcher_menu: FM,
    ) -> Box<dyn Future<Item = Arc<Fetcher>, Error = Error>>
    where
        FK: Into<FetcherKindRaw>,
        FM: Into<FetcherMenu>,
    {
        Box::new(
            future::result(Fetcher::try_new(fetcher_kind_raw, fetcher_menu).map(|e| Arc::new(e)))
                .and_then(move |e| self.fetcher_repository().push(e))
                .and_then(move |e| self.fetcher_sorting_service().push(e)),
        )
    }

    /// new Shifter
    fn new_shifter<FK, FM>(
        &'static self,
        shifter_kind_raw: FK,
        shifter_menu: FM,
    ) -> Box<dyn Future<Item = Arc<Shifter>, Error = Error>>
    where
        FK: Into<ShifterKindRaw>,
        FM: Into<ShifterMenu>,
    {
        Box::new(
            future::result(Shifter::try_new(shifter_kind_raw, shifter_menu).map(|e| Arc::new(e)))
                .and_then(move |e| self.shifter_repository().push(e))
                .and_then(move |e| self.shifter_sorting_service().push(e)),
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

/// Fetcher fetch a file
pub struct Fetcher {
    id: FetcherID,
    kind: FetcherKind,
    menu: FetcherMenu,
}

/// Shifter fetch a file
pub struct Shifter {
    id: ShifterID,
    kind: ShifterKind,
    menu: ShifterMenu,
}

/// Command is a unit of Execution
pub struct Command {
    id: CommandID,
    command_strings: CommandStrings,
    executor_id: ExecutorID,
}

// impl Executor {
//     fn try_new<EK, EM>(executor_kind_raw: EK, executor_menu: EM) -> Result<Self, Error>
//     where
//         EK: Into<ExecutorKindRaw>,
//         EM: Into<ExecutorMenu>,
//     {
//         let kind = executor_kind_raw.into().try_parse()?;
//         let menu = executor_menu.into();
//
//         let id = ExecutorID::try_new(&kind, &menu)?;
//
//         Ok(Self { id, kind, menu })
//     }
//
//     fn kind_of_executor(&self) -> &ExecutorKind {
//         &self.kind
//     }
//
//     fn summary_of_executor<'a>(&'a self) -> (&'a ExecutorID, &'a ExecutorMenu) {
//         (&self.id, &self.menu)
//     }
// }

impl Fetcher {
    fn try_new<FK, FM>(fetcher_kind_raw: FK, fetcher_menu: FM) -> Result<Self, Error>
    where
        FK: Into<FetcherKindRaw>,
        FM: Into<FetcherMenu>,
    {
        let kind = fetcher_kind_raw.into().try_parse()?;
        let menu = fetcher_menu.into();

        let id = FetcherID::try_new(&kind, &menu)?;

        Ok(Self { id, kind, menu })
    }

    fn kind_of_fetcher(&self) -> &FetcherKind {
        &self.kind
    }

    fn summary_of_fetcher<'a>(&'a self) -> (&'a FetcherID, &'a FetcherMenu) {
        (&self.id, &self.menu)
    }
}

impl Shifter {
    fn try_new<FK, FM>(shifter_kind_raw: FK, shifter_menu: FM) -> Result<Self, Error>
    where
        FK: Into<ShifterKindRaw>,
        FM: Into<ShifterMenu>,
    {
        let kind = shifter_kind_raw.into().try_parse()?;
        let menu = shifter_menu.into();

        let id = ShifterID::try_new(&kind, &menu)?;

        Ok(Self { id, kind, menu })
    }

    fn kind_of_shifter(&self) -> &ShifterKind {
        &self.kind
    }

    fn summary_of_shifter<'a>(&'a self) -> (&'a ShifterID, &'a ShifterMenu) {
        (&self.id, &self.menu)
    }
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
    /// Invalid Kind of Fetcher
    InvalidFetcherKind,
    /// Invalid Kind of Shifter
    InvalidShifterKind,
    /// Failed to generate new ExecutorID
    FailedNewExecutorID,
    /// Failed to generate new FetcherID
    FailedNewFetcherID,
    /// Failed to generate new ShifterID
    FailedNewShifterID,
    /// Failed to generate new CommandID
    FailedNewCommandID,
    /// Not enough fetcher menu to generate details
    NotEnoughFetcherMenu(Vec<&'static str>),
    /// Not enough shifter menu to generate details
    NotEnoughShifterMenu(Vec<&'static str>),
}

mod command_id;
mod command_repository;
mod command_strings;
mod command_strings_raw;
mod executor_details;
mod executor_id;
mod executor_kind;
mod executor_repository;
mod fetcher_id;
mod fetcher_kind;
mod fetcher_kind_raw;
mod fetcher_menu;
mod fetcher_repository;
mod fetcher_sorting_service;
mod local_fetcher;
mod local_fetcher_repository;
mod local_source;
mod scp_destination;
mod scp_shifter;
mod scp_shifter_repository;
mod shifter_id;
mod shifter_kind;
mod shifter_kind_raw;
mod shifter_menu;
mod shifter_repository;
mod shifter_sorting_service;
mod ssh_executor;
mod ssh_executor_repository;
mod ssh_host;
mod ssh_user;

// mod executor_kind_raw;

pub(self) use self::command_id::*;
pub(self) use self::command_repository::*;
pub(self) use self::command_strings::*;
pub(self) use self::command_strings_raw::*;
pub(self) use self::executor_details::*;
pub(self) use self::executor_id::*;
pub(self) use self::executor_kind::*;
pub(self) use self::executor_repository::*;
pub(self) use self::fetcher_id::*;
pub(self) use self::fetcher_kind::*;
pub(self) use self::fetcher_kind_raw::*;
pub(self) use self::fetcher_menu::*;
pub(self) use self::fetcher_repository::*;
pub(self) use self::fetcher_sorting_service::*;
pub(self) use self::local_fetcher::*;
pub(self) use self::local_fetcher_repository::*;
pub(self) use self::local_source::*;
pub(self) use self::scp_destination::*;
pub(self) use self::scp_shifter::*;
pub(self) use self::scp_shifter_repository::*;
pub(self) use self::shifter_id::*;
pub(self) use self::shifter_kind::*;
pub(self) use self::shifter_kind_raw::*;
pub(self) use self::shifter_menu::*;
pub(self) use self::shifter_repository::*;
pub(self) use self::shifter_sorting_service::*;
pub(self) use self::ssh_executor::*;
pub(self) use self::ssh_executor_repository::*;
pub(self) use self::ssh_host::*;
pub(self) use self::ssh_user::*;

// pub(self) use self::executor_kind_raw::*;
