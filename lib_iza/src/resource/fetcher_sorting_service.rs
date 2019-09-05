use crate::resource::*;

pub trait FetcherSortingService: LocalFetcherRepositoryComponent {
    fn push(
        &'static self,
        fetcher: Arc<Fetcher>,
    ) -> Box<dyn Future<Item = Arc<Fetcher>, Error = Error>> {
        match fetcher.kind_of_fetcher() {
            FetcherKind::LocalFetcher => Box::new(
                future::result(
                    LocalFetcher::try_new(fetcher.summary_of_fetcher()).map(|l| Arc::new(l)),
                )
                .and_then(move |l| self.local_fetcher_repository().push(l))
                .and_then(move |_| future::ok(fetcher.clone())),
            ),
        }
    }
}

pub trait FetcherSortingServiceComponent {
    type Service: FetcherSortingService;

    fn fetcher_sorting_service(&self) -> &Self::Service;
}
