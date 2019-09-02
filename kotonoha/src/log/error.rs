use futures::prelude::*;
use std::pin::Pin;

pub struct Error {}

pub type ResultFuture<T> = Pin<Box<dyn Future<Output = std::result::Result<T, Error>> + Send>>;
