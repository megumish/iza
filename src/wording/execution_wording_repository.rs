use crate::wording::*;
use std::sync::Arc;

#[cfg(test)]
use futures::prelude::*;
#[cfg(test)]
use std::sync::Mutex;

pub trait ExecutionWordingRepository {
    fn push(&'static self, wording: Arc<ExecutionWording>) -> ResultFuture<Arc<ExecutionWording>>;
}

pub trait HasExecutionWordingRepository {
    type Repository: ExecutionWordingRepository;

    fn execution_wording_repository(&self) -> &Self::Repository;
}

#[cfg(test)]
pub struct InMemoryExecutionWordingDatabase {
    wordings: Arc<Mutex<Vec<ExecutionWording>>>,
}

#[cfg(test)]
impl InMemoryExecutionWordingDatabase {
    pub fn new() -> Self {
        Self {
            wordings: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[cfg(test)]
impl ExecutionWordingRepository for InMemoryExecutionWordingDatabase {
    fn push(&'static self, wording: Arc<ExecutionWording>) -> ResultFuture<Arc<ExecutionWording>> {
        {
            let wording = wording.clone();
            let wordings = self.wordings.clone();
            let wordings = wordings
                .lock()
                .expect("Can not get ExecutionWording from InMemoryDatabase.");
            match wordings.iter().find(|w| **w == (&*wording).clone()) {
                Some(_) => {
                    return future::ready(Err(ErrorKind::AlreadyExistsExecutionWording.into()))
                        .boxed()
                }
                None => { /* do nothing */ }
            }
        }
        {
            let wordings = self.wordings.clone();
            let mut wordings = wordings
                .lock()
                .expect("Can not get ExecutorWording from InMemoryDatabase.");
            wordings.push((&*wording).clone());
            future::ok(wording).boxed()
        }
    }
}
