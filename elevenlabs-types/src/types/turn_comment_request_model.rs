pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TurnCommentRequestModel {
    /// Zero-based index of the transcript turn this comment refers to.
    #[serde(default)]
    pub turn_index: i64,
    /// What went wrong at this turn.
    #[serde(default)]
    pub comment: String,
}

impl TurnCommentRequestModel {
    pub fn builder() -> TurnCommentRequestModelBuilder {
        <TurnCommentRequestModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TurnCommentRequestModelBuilder {
    turn_index: Option<i64>,
    comment: Option<String>,
}

impl TurnCommentRequestModelBuilder {
    pub fn turn_index(mut self, value: i64) -> Self {
        self.turn_index = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TurnCommentRequestModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`turn_index`](TurnCommentRequestModelBuilder::turn_index)
    /// - [`comment`](TurnCommentRequestModelBuilder::comment)
    pub fn build(self) -> Result<TurnCommentRequestModel, BuildError> {
        Ok(TurnCommentRequestModel {
            turn_index: self.turn_index.ok_or_else(|| BuildError::missing_field("turn_index"))?,
            comment: self.comment.ok_or_else(|| BuildError::missing_field("comment"))?,
        })
    }
}
