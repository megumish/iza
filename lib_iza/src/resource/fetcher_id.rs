use crate::resource::*;

#[derive(Clone, Serialize)]
pub struct FetcherID {
    id_string: String,
}

impl FetcherID {
    pub fn try_new(kind: &FetcherKind, menu: &FetcherMenu) -> Result<Self, Error> {
        use blake2::Digest;

        #[derive(Serialize)]
        struct SerdeObject<'a> {
            kind: &'a FetcherKind,
            menu: &'a FetcherMenu,
        }

        let obj = SerdeObject { kind, menu };

        let serialized = serde_json::to_vec(&obj).map_err(|_| Error::FailedNewFetcherID)?;

        let digest = blake2::Blake2b::digest(&serialized);

        Ok(Self {
            id_string: hex::encode(digest),
        })
    }
}
