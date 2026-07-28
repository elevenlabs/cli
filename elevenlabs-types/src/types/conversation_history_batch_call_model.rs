pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationHistoryBatchCallModel {
    #[serde(default)]
    pub batch_call_id: String,
    #[serde(default)]
    pub batch_call_recipient_id: String,
}

impl ConversationHistoryBatchCallModel {
    pub fn builder() -> ConversationHistoryBatchCallModelBuilder {
        <ConversationHistoryBatchCallModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryBatchCallModelBuilder {
    batch_call_id: Option<String>,
    batch_call_recipient_id: Option<String>,
}

impl ConversationHistoryBatchCallModelBuilder {
    pub fn batch_call_id(mut self, value: impl Into<String>) -> Self {
        self.batch_call_id = Some(value.into());
        self
    }

    pub fn batch_call_recipient_id(mut self, value: impl Into<String>) -> Self {
        self.batch_call_recipient_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryBatchCallModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`batch_call_id`](ConversationHistoryBatchCallModelBuilder::batch_call_id)
    /// - [`batch_call_recipient_id`](ConversationHistoryBatchCallModelBuilder::batch_call_recipient_id)
    pub fn build(self) -> Result<ConversationHistoryBatchCallModel, BuildError> {
        Ok(ConversationHistoryBatchCallModel {
            batch_call_id: self.batch_call_id.ok_or_else(|| BuildError::missing_field("batch_call_id"))?,
            batch_call_recipient_id: self.batch_call_recipient_id.ok_or_else(|| BuildError::missing_field("batch_call_recipient_id"))?,
        })
    }
}
