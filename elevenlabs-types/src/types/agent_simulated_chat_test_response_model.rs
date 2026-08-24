pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentSimulatedChatTestResponseModel {
    #[serde(default)]
    pub simulated_conversation: Vec<ConversationHistoryTranscriptResponseModel>,
    pub analysis: ConversationHistoryAnalysisCommonModel,
}

impl AgentSimulatedChatTestResponseModel {
    pub fn builder() -> AgentSimulatedChatTestResponseModelBuilder {
        <AgentSimulatedChatTestResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSimulatedChatTestResponseModelBuilder {
    simulated_conversation: Option<Vec<ConversationHistoryTranscriptResponseModel>>,
    analysis: Option<ConversationHistoryAnalysisCommonModel>,
}

impl AgentSimulatedChatTestResponseModelBuilder {
    pub fn simulated_conversation(mut self, value: Vec<ConversationHistoryTranscriptResponseModel>) -> Self {
        self.simulated_conversation = Some(value);
        self
    }

    pub fn analysis(mut self, value: ConversationHistoryAnalysisCommonModel) -> Self {
        self.analysis = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentSimulatedChatTestResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`simulated_conversation`](AgentSimulatedChatTestResponseModelBuilder::simulated_conversation)
    /// - [`analysis`](AgentSimulatedChatTestResponseModelBuilder::analysis)
    pub fn build(self) -> Result<AgentSimulatedChatTestResponseModel, BuildError> {
        Ok(AgentSimulatedChatTestResponseModel {
            simulated_conversation: self.simulated_conversation.ok_or_else(|| BuildError::missing_field("simulated_conversation"))?,
            analysis: self.analysis.ok_or_else(|| BuildError::missing_field("analysis"))?,
        })
    }
}
