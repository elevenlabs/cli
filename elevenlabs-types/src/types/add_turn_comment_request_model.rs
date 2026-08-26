pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddTurnCommentRequestModel {
    /// Zero-based index of the transcript turn this comment refers to.
    #[serde(default)]
    pub turn_index: i64,
    /// What went wrong at this turn.
    #[serde(default)]
    pub comment: String,
}

impl AddTurnCommentRequestModel {
    pub fn builder() -> AddTurnCommentRequestModelBuilder {
        <AddTurnCommentRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddTurnCommentRequestModelBuilder {
    turn_index: Option<i64>,
    comment: Option<String>,
}

impl AddTurnCommentRequestModelBuilder {
    pub fn turn_index(mut self, value: i64) -> Self {
        self.turn_index = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddTurnCommentRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`turn_index`](AddTurnCommentRequestModelBuilder::turn_index)
    /// - [`comment`](AddTurnCommentRequestModelBuilder::comment)
    pub fn build(self) -> Result<AddTurnCommentRequestModel, BuildError> {
        Ok(AddTurnCommentRequestModel {
            turn_index: self.turn_index.ok_or_else(|| BuildError::missing_field("turn_index"))?,
            comment: self.comment.ok_or_else(|| BuildError::missing_field("comment"))?,
        })
    }
}

