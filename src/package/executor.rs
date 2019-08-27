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

pub struct ExecutorLeaf {
    pub packages: Vec<Package>,
}

impl Executor {
    pub fn new(name: ExecutorName) -> Self {
        let inner = Arc::new(ExecutorLeaf {
            packages: Vec::new(),
        });
        Self { name, inner }
    }

    pub fn execute(&self) -> ResultFuture<Vec<Arc<Execution>>> {
        self.inner.execute()
    }
}

impl ExecutorDetail for ExecutorLeaf {
    fn execute(&self) -> ResultFuture<Vec<Arc<Execution>>> {
        Execution::new_vec_arc(&self.packages)
    }
}
