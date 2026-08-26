pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TurnCommentResponseModel {
    #[serde(default)]
    pub turn_index: i64,
    #[serde(default)]
    pub comment: String,
    #[serde(default)]
    pub created_at_unix_secs: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
}

impl TurnCommentResponseModel {
    pub fn builder() -> TurnCommentResponseModelBuilder {
        <TurnCommentResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TurnCommentResponseModelBuilder {
    turn_index: Option<i64>,
    comment: Option<String>,
    created_at_unix_secs: Option<i64>,
    owner_user_id: Option<String>,
}

impl TurnCommentResponseModelBuilder {
    pub fn turn_index(mut self, value: i64) -> Self {
        self.turn_index = Some(value);
        self
    }

    pub fn comment(mut self, value: impl Into<String>) -> Self {
        self.comment = Some(value.into());
        self
    }

    pub fn created_at_unix_secs(mut self, value: i64) -> Self {
        self.created_at_unix_secs = Some(value);
        self
    }

    pub fn owner_user_id(mut self, value: impl Into<String>) -> Self {
        self.owner_user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TurnCommentResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`turn_index`](TurnCommentResponseModelBuilder::turn_index)
    /// - [`comment`](TurnCommentResponseModelBuilder::comment)
    /// - [`created_at_unix_secs`](TurnCommentResponseModelBuilder::created_at_unix_secs)
    pub fn build(self) -> Result<TurnCommentResponseModel, BuildError> {
        Ok(TurnCommentResponseModel {
            turn_index: self.turn_index.ok_or_else(|| BuildError::missing_field("turn_index"))?,
            comment: self.comment.ok_or_else(|| BuildError::missing_field("comment"))?,
            created_at_unix_secs: self.created_at_unix_secs.ok_or_else(|| BuildError::missing_field("created_at_unix_secs"))?,
            owner_user_id: self.owner_user_id,
        })
    }
}
