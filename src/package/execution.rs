use crate::package::*;
use futures::prelude::*;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct Execution {
    pub id: ExecutionID,
    pub name: ExecutionName,
}

impl Execution {
    pub fn new_vec_arc(packages: &Vec<Package>) -> ResultFuture<Vec<Arc<Execution>>> {
        future::ok(
            packages
                .iter()
                .map(|p| {
                    Arc::new(Execution {
                        id: uuid::Uuid::new_v4().to_hyphenated().to_string().into(),
                        name: p.name.to_string().into(),
                    })
                })
                .collect(),
        )
        .boxed()
    }
}
