pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AttentionPageResponse {
    #[serde(default)]
    pub attention: Vec<AttentionView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl AttentionPageResponse {
    pub fn builder() -> AttentionPageResponseBuilder {
        <AttentionPageResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttentionPageResponseBuilder {
    attention: Option<Vec<AttentionView>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl AttentionPageResponseBuilder {
    pub fn attention(mut self, value: Vec<AttentionView>) -> Self {
        self.attention = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AttentionPageResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attention`](AttentionPageResponseBuilder::attention)
    /// - [`has_more`](AttentionPageResponseBuilder::has_more)
    pub fn build(self) -> Result<AttentionPageResponse, BuildError> {
        Ok(AttentionPageResponse {
            attention: self.attention.ok_or_else(|| BuildError::missing_field("attention"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
