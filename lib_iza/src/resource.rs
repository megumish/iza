//! Resource Domain represent interface of deployment resources
use futures::{future, prelude::*};
use std::sync::Arc;

/// Resource App is a interface for library user.
pub trait ResourceApp: ExecutorRepositoryComponent + ExecutorSortingServiceComponent {
    /// new Executor
    fn new_executor<EK, EM>(
        &'static self,
        executor_kind_raw: EK,
        executor_menu: EM,
    ) -> Box<dyn Future<Item = Arc<Executor>, Error = Error>>
    where
        EK: Into<ExecutorKindRaw>,
        EM: Into<ExecutorMenu>,
    {
        Box::new(
            future::result(
                Executor::try_new(executor_kind_raw, executor_menu).map(|e| Arc::new(e)),
            )
            .and_then(move |e| self.executor_repository().push(e))
            .and_then(move |e| self.executor_sorting_service().push(e)),
        )
    }
}

/// Executor execute a command
pub struct Executor {
    id: ExecutorID,
    kind: ExecutorKind,
    menu: ExecutorMenu,
}

impl Executor {
    fn try_new<EK, EM>(executor_kind_raw: EK, executor_menu: EM) -> Result<Self, Error>
    where
        EK: Into<ExecutorKindRaw>,
        EM: Into<ExecutorMenu>,
    {
        let kind = executor_kind_raw.into().parse()?;
        let menu = executor_menu.into();

        let id = ExecutorID::try_new(&kind, &menu)?;

        Ok(Self { id, kind, menu })
    }

    fn kind_of_executor(&self) -> &ExecutorKind {
        &self.kind
    }

    fn summary_of_executor<'a>(&'a self) -> (&'a ExecutorID, &'a ExecutorMenu) {
        (&self.id, &self.menu)
    }
}

/// Resource Error
pub enum Error {
    /// Invalid Kind of Executor
    InvalidExecutorKind,
    /// Failed to generate new ExecutorID
    FailedNewExecutorID,
    /// Not enough executor menu to generate details
    NotEnoughExecutorMenu(Vec<&'static str>),
}

mod executor_id;
mod executor_kind;
mod executor_kind_raw;
mod executor_menu;
mod executor_repository;
mod executor_sorting_service;
mod ssh_executor;
mod ssh_executor_repository;
mod ssh_host;
mod ssh_user;

pub(self) use self::executor_id::*;
pub(self) use self::executor_kind::*;
pub(self) use self::executor_kind_raw::*;
pub(self) use self::executor_menu::*;
pub(self) use self::executor_repository::*;
pub(self) use self::executor_sorting_service::*;
pub(self) use self::ssh_executor::*;
pub(self) use self::ssh_executor_repository::*;
pub(self) use self::ssh_host::*;
pub(self) use self::ssh_user::*;
