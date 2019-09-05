use crate::resource::*;

pub struct FetcherKindRaw {
    raw_string: &'static str,
}

impl FetcherKindRaw {
    pub fn try_parse(self) -> Result<FetcherKind, Error> {
        match self.raw_string {
            "LocalFetcher" => Ok(FetcherKind::LocalFetcher),
            _ => Err(Error::InvalidFetcherKind),
        }
    }
}
