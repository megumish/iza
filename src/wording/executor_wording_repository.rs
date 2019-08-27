use crate::wording::*;
use std::sync::Arc;

#[cfg(test)]
use futures::prelude::*;
#[cfg(test)]
use std::sync::Mutex;

pub trait ExecutorWordingRepository {
    fn push(&self, wording: Arc<ExecutorWording>) -> ResultFuture<Arc<ExecutorWording>>;
}

pub trait HasExecutorWordingRepository {
    type Repository: ExecutorWordingRepository;

    fn executor_wording_repository(&self) -> &Self::Repository;
}

#[cfg(test)]
pub struct InMemoryExecutorWordingDatabase {
    wordings: Arc<Mutex<Vec<ExecutorWording>>>,
}

#[cfg(test)]
impl InMemoryExecutorWordingDatabase {
    pub fn new() -> Self {
        Self {
            wordings: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[cfg(test)]
impl ExecutorWordingRepository for InMemoryExecutorWordingDatabase {
    fn push(&self, wording: Arc<ExecutorWording>) -> ResultFuture<Arc<ExecutorWording>> {
        {
            let wording = wording.clone();
            let wordings = self.wordings.clone();
            let wordings = wordings
                .lock()
                .expect("Can not get ExecutorWording from InMemoryDatabase.");
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
