use crate::resource::*;

#[derive(Clone, Serialize)]
pub struct ShifterID {
    id_string: String,
}

impl ShifterID {
    pub fn try_new(kind: &ShifterKind, menu: &ShifterMenu) -> Result<Self, Error> {
        use blake2::Digest;

        #[derive(Serialize)]
        struct SerdeObject<'a> {
            kind: &'a ShifterKind,
            menu: &'a ShifterMenu,
        }

        let obj = SerdeObject { kind, menu };

        let serialized = serde_json::to_vec(&obj).map_err(|_| Error::FailedNewShifterID)?;

        let digest = blake2::Blake2b::digest(&serialized);

        Ok(Self {
            id_string: hex::encode(digest),
        })
    }
}
