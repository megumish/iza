use crate::resource::*;
use futures::{future, prelude::*};

pub trait ExecuteCommandService: CommandRepositoryComponent + ExecutionRepositoryComponent {
    fn execute_commands_of_ids<CID>(
        &'static self,
        command_ids: Vec<CID>,
    ) -> Box<Future<Item = Vec<Arc<Execution>>, Error = Error>>
    where
        CID: Into<CommandID>,
    {
        Box::new(
            self.command_repository()
                .commands_of_ids(command_ids)
                .and_then(|cs| execute_commands(self, cs)),
        )
    }
}

fn execute_commands<S>(suite: &'static S, commands: Vec<Command>) -> impl Future
where
    S: ExecutionRepositoryComponent,
{
    future::join_all(commands.iter().map(|c| {
        future::lazy(|| c.new_execution()).and_then(|e| suite.execution_repository().push(e))
    }))
}
