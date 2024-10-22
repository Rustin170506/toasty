use super::*;

use toasty_core::schema::FieldId;

#[derive(Debug)]
pub(crate) struct Associate {
    /// The variable that holds the source records
    pub(crate) source: plan::VarId,

    /// The variable that holds the target records
    pub(crate) target: plan::VarId,

    /// The source field being associated
    pub(crate) field: FieldId,
}

impl<'stmt> From<Associate> for Action<'stmt> {
    fn from(value: Associate) -> Self {
        Action::Associate(value)
    }
}