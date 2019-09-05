use crate::resource::*;

pub struct CommandID {
    id_string: String,
}

impl CommandID {
    pub fn try_new(
        command_strings: &CommandStrings,
        executor_id: &ExecutorID,
    ) -> Result<Self, Error> {
        use blake2::Digest;

        #[derive(Serialize)]
        struct SerdeObject<'a> {
            command_strings: &'a CommandStrings,
            executor_id: &'a ExecutorID,
        }

        let obj = SerdeObject {
            command_strings,
            executor_id,
        };

        let serialized = serde_json::to_vec(&obj).map_err(|_| Error::FailedNewCommandID)?;

        let digest = blake2::Blake2b::digest(&serialized);

        Ok(Self {
            id_string: hex::encode(digest),
        })
    }
}
