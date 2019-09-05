use crate::resource::*;

#[derive(Clone)]
pub struct ExecutorID {
    id_string: String,
}

impl ExecutorID {
    pub fn try_new(kind: &ExecutorKind, menu: &ExecutorMenu) -> Result<Self, Error> {
        use blake2::Digest;

        #[derive(Serialize)]
        struct SerdeObject<'a> {
            kind: &'a ExecutorKind,
            menu: &'a ExecutorMenu,
        }

        let obj = SerdeObject { kind, menu };

        let serialized = serde_json::to_vec(&obj).map_err(|_| Error::FailedNewExecutorID)?;

        let digest = blake2::Blake2b::digest(&serialized);

        Ok(Self {
            id_string: hex::encode(digest),
        })
    }
}
