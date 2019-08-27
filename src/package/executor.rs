use crate::package::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Executor {
    pub name: ExecutorName,
    inner: Arc<dyn ExecutorDetail>,
}

pub trait ExecutorDetail {
    fn execute(&self) -> ResultFuture<Vec<Arc<Execution>>>;
}

struct ExecutorLeaf {
    packages: Vec<Package>,
}

impl Executor {
    pub fn execute(&self) -> ResultFuture<Vec<Arc<Execution>>> {
        self.inner.execute()
    }
}

impl ExecutorDetail for ExecutorLeaf {
    fn execute(&self) -> ResultFuture<Vec<Arc<Execution>>> {
        Execution::new_vec_arc(&self.packages)
    }
}
