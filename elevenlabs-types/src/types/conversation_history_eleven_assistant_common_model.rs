pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationHistoryElevenAssistantCommonModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eleven_assistant: Option<bool>,
}

impl ConversationHistoryElevenAssistantCommonModel {
    pub fn builder() -> ConversationHistoryElevenAssistantCommonModelBuilder {
        <ConversationHistoryElevenAssistantCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryElevenAssistantCommonModelBuilder {
    is_eleven_assistant: Option<bool>,
}

impl ConversationHistoryElevenAssistantCommonModelBuilder {
    pub fn is_eleven_assistant(mut self, value: bool) -> Self {
        self.is_eleven_assistant = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryElevenAssistantCommonModel`].
    pub fn build(self) -> Result<ConversationHistoryElevenAssistantCommonModel, BuildError> {
        Ok(ConversationHistoryElevenAssistantCommonModel {
            is_eleven_assistant: self.is_eleven_assistant,
        })
    }
}
