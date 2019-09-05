use crate::resource::*;
use std::sync::Arc;

pub trait FetcherRepository {
    fn push(
        &'static self,
        fetcher: Arc<Fetcher>,
    ) -> Box<dyn Future<Item = Arc<Fetcher>, Error = Error>>;
}

pub trait FetcherRepositoryComponent {
    type Repository: FetcherRepository;

    fn fetcher_repository(&self) -> &Self::Repository;
}
