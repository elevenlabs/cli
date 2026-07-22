pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetConversationResponseModel {
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_product: Option<String>,
    pub status: GetConversationResponseModelStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// The ID of the agent version used for this conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(default)]
    pub metadata: ConversationHistoryMetadataCommonModel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<ConversationHistoryAnalysisCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visited_agents: Option<Vec<VisitedAgentRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub has_audio: bool,
    #[serde(default)]
    pub has_user_audio: bool,
    #[serde(default)]
    pub has_response_audio: bool,
    #[serde(default)]
    pub has_auxiliary_audio: bool,
    #[serde(default)]
    pub transcript: Vec<ConversationHistoryTranscriptResponseModel>,
    /// Conversation tag ids assigned to this conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    /// OpenTelemetry trace payload when the request uses format=opentelemetry; otherwise omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otlp_traces: Option<HashMap<String, serde_json::Value>>,
}

impl GetConversationResponseModel {
    pub fn builder() -> GetConversationResponseModelBuilder {
        <GetConversationResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetConversationResponseModelBuilder {
    agent_id: Option<String>,
    agent_name: Option<String>,
    conversation_product: Option<String>,
    status: Option<GetConversationResponseModelStatus>,
    user_id: Option<String>,
    branch_id: Option<String>,
    version_id: Option<String>,
    metadata: Option<ConversationHistoryMetadataCommonModel>,
    analysis: Option<ConversationHistoryAnalysisCommonModel>,
    visited_agents: Option<Vec<VisitedAgentRef>>,
    conversation_initiation_client_data: Option<ConversationInitiationClientDataRequestOutput>,
    environment: Option<String>,
    conversation_id: Option<String>,
    has_audio: Option<bool>,
    has_user_audio: Option<bool>,
    has_response_audio: Option<bool>,
    has_auxiliary_audio: Option<bool>,
    transcript: Option<Vec<ConversationHistoryTranscriptResponseModel>>,
    tag_ids: Option<Vec<String>>,
    otlp_traces: Option<HashMap<String, serde_json::Value>>,
}

impl GetConversationResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn conversation_product(mut self, value: impl Into<String>) -> Self {
        self.conversation_product = Some(value.into());
        self
    }

    pub fn status(mut self, value: GetConversationResponseModelStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: ConversationHistoryMetadataCommonModel) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn analysis(mut self, value: ConversationHistoryAnalysisCommonModel) -> Self {
        self.analysis = Some(value);
        self
    }

    pub fn visited_agents(mut self, value: Vec<VisitedAgentRef>) -> Self {
        self.visited_agents = Some(value);
        self
    }

    pub fn conversation_initiation_client_data(mut self, value: ConversationInitiationClientDataRequestOutput) -> Self {
        self.conversation_initiation_client_data = Some(value);
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn has_audio(mut self, value: bool) -> Self {
        self.has_audio = Some(value);
        self
    }

    pub fn has_user_audio(mut self, value: bool) -> Self {
        self.has_user_audio = Some(value);
        self
    }

    pub fn has_response_audio(mut self, value: bool) -> Self {
        self.has_response_audio = Some(value);
        self
    }

    pub fn has_auxiliary_audio(mut self, value: bool) -> Self {
        self.has_auxiliary_audio = Some(value);
        self
    }

    pub fn transcript(mut self, value: Vec<ConversationHistoryTranscriptResponseModel>) -> Self {
        self.transcript = Some(value);
        self
    }

    pub fn tag_ids(mut self, value: Vec<String>) -> Self {
        self.tag_ids = Some(value);
        self
    }

    pub fn otlp_traces(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.otlp_traces = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetConversationResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](GetConversationResponseModelBuilder::agent_id)
    /// - [`status`](GetConversationResponseModelBuilder::status)
    /// - [`metadata`](GetConversationResponseModelBuilder::metadata)
    /// - [`conversation_id`](GetConversationResponseModelBuilder::conversation_id)
    /// - [`has_audio`](GetConversationResponseModelBuilder::has_audio)
    /// - [`has_user_audio`](GetConversationResponseModelBuilder::has_user_audio)
    /// - [`has_response_audio`](GetConversationResponseModelBuilder::has_response_audio)
    /// - [`has_auxiliary_audio`](GetConversationResponseModelBuilder::has_auxiliary_audio)
    /// - [`transcript`](GetConversationResponseModelBuilder::transcript)
    pub fn build(self) -> Result<GetConversationResponseModel, BuildError> {
        Ok(GetConversationResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            agent_name: self.agent_name,
            conversation_product: self.conversation_product,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            user_id: self.user_id,
            branch_id: self.branch_id,
            version_id: self.version_id,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            analysis: self.analysis,
            visited_agents: self.visited_agents,
            conversation_initiation_client_data: self.conversation_initiation_client_data,
            environment: self.environment,
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            has_audio: self.has_audio.ok_or_else(|| BuildError::missing_field("has_audio"))?,
            has_user_audio: self.has_user_audio.ok_or_else(|| BuildError::missing_field("has_user_audio"))?,
            has_response_audio: self.has_response_audio.ok_or_else(|| BuildError::missing_field("has_response_audio"))?,
            has_auxiliary_audio: self.has_auxiliary_audio.ok_or_else(|| BuildError::missing_field("has_auxiliary_audio"))?,
            transcript: self.transcript.ok_or_else(|| BuildError::missing_field("transcript"))?,
            tag_ids: self.tag_ids,
            otlp_traces: self.otlp_traces,
        })
    }
}
