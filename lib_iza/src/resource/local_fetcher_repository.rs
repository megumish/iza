use crate::resource::*;
use std::sync::Arc;

pub trait LocalFetcherRepository {
    fn push(
        &'static self,
        local_fetcher: Arc<LocalFetcher>,
    ) -> Box<dyn Future<Item = Arc<LocalFetcher>, Error = Error>>;
}

pub trait LocalFetcherRepositoryComponent {
    type Repository: LocalFetcherRepository;

    fn local_fetcher_repository(&self) -> &Self::Repository;
}
