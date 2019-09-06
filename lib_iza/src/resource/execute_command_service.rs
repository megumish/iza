use crate::resource::*;
use futures::future;

pub trait ExecuteCommandService:
    CommandRepositoryComponent + ExecutionRepositoryComponent + ExecutorRepositoryComponent
{
    fn execute_commands_of_ids<CID>(
        &'static self,
        command_ids: Vec<CID>,
    ) -> Box<dyn Future<Item = Vec<Arc<Execution<Box<dyn FnOnce()>>>>, Error = Error>>
    where
        CID: Into<CommandID>,
        Self: Sized,
    {
        Box::new(
            self.command_repository()
                .commands_of_ids(command_ids)
                .and_then(move |cs| execute_commands(self, cs)),
        )
    }
}

pub trait ExecuteCommandServiceComponent {
    type Service: ExecuteCommandService;

    fn execute_command_service(&self) -> &Self::Service;
}

fn execute_commands<S>(
    suite: &'static S,
    commands: Vec<Arc<Command>>,
) -> Box<dyn Future<Item = Vec<Arc<Execution<Box<dyn FnOnce()>>>>, Error = Error>>
where
    S: ExecutionRepositoryComponent + ExecutorRepositoryComponent,
{
    Box::new(future::join_all(commands.into_iter().map(move |c| {
        future::lazy(move || (&*c).clone().new_command_execution(suite))
            .and_then(move |e| suite.execution_repository().push(e))
    })))
}
