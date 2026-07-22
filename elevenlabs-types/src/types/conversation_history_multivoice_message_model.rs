pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Represents a message from a multi-voice agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationHistoryMultivoiceMessageModel {
    #[serde(default)]
    pub parts: Vec<ConversationHistoryMultivoiceMessagePartModel>,
}

impl ConversationHistoryMultivoiceMessageModel {
    pub fn builder() -> ConversationHistoryMultivoiceMessageModelBuilder {
        <ConversationHistoryMultivoiceMessageModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryMultivoiceMessageModelBuilder {
    parts: Option<Vec<ConversationHistoryMultivoiceMessagePartModel>>,
}

impl ConversationHistoryMultivoiceMessageModelBuilder {
    pub fn parts(mut self, value: Vec<ConversationHistoryMultivoiceMessagePartModel>) -> Self {
        self.parts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryMultivoiceMessageModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`parts`](ConversationHistoryMultivoiceMessageModelBuilder::parts)
    pub fn build(self) -> Result<ConversationHistoryMultivoiceMessageModel, BuildError> {
        Ok(ConversationHistoryMultivoiceMessageModel {
            parts: self.parts.ok_or_else(|| BuildError::missing_field("parts"))?,
        })
    }
}
