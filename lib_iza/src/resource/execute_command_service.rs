use crate::resource::*;
use futures::{future, prelude::*};

pub trait ExecuteCommandService: CommandRepositoryComponent + ExecutionRepositoryComponent {
    fn execute_commands_of_ids<CID, F>(&'static self, command_ids: Vec<CID>) -> F
    where
        F: Future<Item = Vec<Arc<Execution>>, Error = Error>,
    {
        Box::new(
            self.command_repository()
                .commands_of_ids(command_ids)
                .and_then(|cs| {
                    future::join_all({
                        cs.iter().map(|c| {
                            future::lazy(|_| {
                                let execution = c.new_execution();
                                self.execution_repository().push(execution);
                            })
                        })
                    })
                }),
        )
    }
}
