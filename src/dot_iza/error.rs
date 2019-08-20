pub struct Error {}

pub type ResultFuture<T> =
    std::pin::Pin<Box<dyn futures::Future<Output = std::result::Result<T, Error>> + Send>>;
