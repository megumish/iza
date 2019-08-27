pub use crate::wording::*;

#[derive(Debug, PartialEq, Clone)]
pub struct ExecutionWording {
    pub id: ExecutionID,
    pub execution_name: ExecutionName,
}
