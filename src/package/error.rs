pub enum Error {
    Unimplemented,
}

pub(super) type Result<T> = std::result::Result<T, Error>;
