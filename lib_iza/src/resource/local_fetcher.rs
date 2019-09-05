use crate::resource::*;

pub struct LocalFetcher {
    id: FetcherID,
    source: LocalSource,
}

impl LocalFetcher {
    pub fn try_new((id, menu): (&FetcherID, &FetcherMenu)) -> Result<Self, Error> {
        let (source,) = menu.parse_local_menu()?;

        let id = (*id).clone();
        Ok(Self { id, source })
    }
}
