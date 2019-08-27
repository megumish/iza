use crate::package::*;
use futures::prelude::*;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct Execution {
    pub name: ExecutionName,
}

impl Execution {
    pub fn new_vec_arc(packages: &Vec<Package>) -> ResultFuture<Vec<Arc<Execution>>> {
        future::ok(
            packages
                .iter()
                .map(|p| {
                    Arc::new(Execution {
                        name: p.name.to_string().into(),
                    })
                })
                .collect(),
        )
        .boxed()
    }
}
