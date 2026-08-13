pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationHistoryRagUsageCommonModel {
    #[serde(default)]
    pub usage_count: i64,
    #[serde(default)]
    pub embedding_model: String,
}

impl ConversationHistoryRagUsageCommonModel {
    pub fn builder() -> ConversationHistoryRagUsageCommonModelBuilder {
        <ConversationHistoryRagUsageCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryRagUsageCommonModelBuilder {
    usage_count: Option<i64>,
    embedding_model: Option<String>,
}

impl ConversationHistoryRagUsageCommonModelBuilder {
    pub fn usage_count(mut self, value: i64) -> Self {
        self.usage_count = Some(value);
        self
    }

    pub fn embedding_model(mut self, value: impl Into<String>) -> Self {
        self.embedding_model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryRagUsageCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`usage_count`](ConversationHistoryRagUsageCommonModelBuilder::usage_count)
    /// - [`embedding_model`](ConversationHistoryRagUsageCommonModelBuilder::embedding_model)
    pub fn build(self) -> Result<ConversationHistoryRagUsageCommonModel, BuildError> {
        Ok(ConversationHistoryRagUsageCommonModel {
            usage_count: self.usage_count.ok_or_else(|| BuildError::missing_field("usage_count"))?,
            embedding_model: self.embedding_model.ok_or_else(|| BuildError::missing_field("embedding_model"))?,
        })
    }
}
