pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationReasoningModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_redact: Option<bool>,
}

impl ConversationReasoningModel {
    pub fn builder() -> ConversationReasoningModelBuilder {
        <ConversationReasoningModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationReasoningModelBuilder {
    summary: Option<String>,
    provider_redact: Option<bool>,
}

impl ConversationReasoningModelBuilder {
    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn provider_redact(mut self, value: bool) -> Self {
        self.provider_redact = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationReasoningModel`].
    pub fn build(self) -> Result<ConversationReasoningModel, BuildError> {
        Ok(ConversationReasoningModel {
            summary: self.summary,
            provider_redact: self.provider_redact,
        })
    }
}
