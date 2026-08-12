pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationHistoryMetadataCommonModel {
    #[serde(default)]
    pub start_time_unix_secs: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_time_unix_secs: Option<i64>,
    #[serde(default)]
    pub call_duration_secs: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_settings: Option<ConversationDeletionSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<ConversationHistoryFeedbackCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_method: Option<AuthorizationMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging: Option<ConversationChargingCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_call: Option<ConversationHistoryMetadataCommonModelPhoneCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_call: Option<ConversationHistoryBatchCallModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ConversationHistoryErrorCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rag_usage: Option<ConversationHistoryRagUsageCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features_usage: Option<FeaturesUsageCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eleven_assistant: Option<ConversationHistoryElevenAssistantCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_source: Option<ConversationInitiationSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_initiation_source_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_metadata: Option<AsyncConversationMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp: Option<WhatsAppConversationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms: Option<SmsConversationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_created_from: Option<AgentDefinitionSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_last_updated_from: Option<AgentDefinitionSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_rewards: Option<Vec<ConversationVoiceRewardModel>>,
    /// Total fiat cost of the conversation in USD, i.e. the sum of the LLM price and the non-LLM platform price (the fiat analogue of ``cost``). ``None`` when neither is set (e.g. conversations that predate fiat cost tracking).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_fiat: Option<f64>,
}

impl ConversationHistoryMetadataCommonModel {
    pub fn builder() -> ConversationHistoryMetadataCommonModelBuilder {
        <ConversationHistoryMetadataCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryMetadataCommonModelBuilder {
    start_time_unix_secs: Option<i64>,
    accepted_time_unix_secs: Option<i64>,
    call_duration_secs: Option<i64>,
    cost: Option<i64>,
    deletion_settings: Option<ConversationDeletionSettings>,
    feedback: Option<ConversationHistoryFeedbackCommonModel>,
    authorization_method: Option<AuthorizationMethod>,
    charging: Option<ConversationChargingCommonModel>,
    phone_call: Option<ConversationHistoryMetadataCommonModelPhoneCall>,
    batch_call: Option<ConversationHistoryBatchCallModel>,
    termination_reason: Option<String>,
    error: Option<ConversationHistoryErrorCommonModel>,
    warnings: Option<Vec<String>>,
    main_language: Option<String>,
    rag_usage: Option<ConversationHistoryRagUsageCommonModel>,
    text_only: Option<bool>,
    features_usage: Option<FeaturesUsageCommonModel>,
    eleven_assistant: Option<ConversationHistoryElevenAssistantCommonModel>,
    initiator_id: Option<String>,
    conversation_initiation_source: Option<ConversationInitiationSource>,
    conversation_initiation_source_version: Option<String>,
    timezone: Option<String>,
    async_metadata: Option<AsyncConversationMetadata>,
    whatsapp: Option<WhatsAppConversationInfo>,
    sms: Option<SmsConversationInfo>,
    agent_created_from: Option<AgentDefinitionSource>,
    agent_last_updated_from: Option<AgentDefinitionSource>,
    voice_rewards: Option<Vec<ConversationVoiceRewardModel>>,
    cost_fiat: Option<f64>,
}

impl ConversationHistoryMetadataCommonModelBuilder {
    pub fn start_time_unix_secs(mut self, value: i64) -> Self {
        self.start_time_unix_secs = Some(value);
        self
    }

    pub fn accepted_time_unix_secs(mut self, value: i64) -> Self {
        self.accepted_time_unix_secs = Some(value);
        self
    }

    pub fn call_duration_secs(mut self, value: i64) -> Self {
        self.call_duration_secs = Some(value);
        self
    }

    pub fn cost(mut self, value: i64) -> Self {
        self.cost = Some(value);
        self
    }

    pub fn deletion_settings(mut self, value: ConversationDeletionSettings) -> Self {
        self.deletion_settings = Some(value);
        self
    }

    pub fn feedback(mut self, value: ConversationHistoryFeedbackCommonModel) -> Self {
        self.feedback = Some(value);
        self
    }

    pub fn authorization_method(mut self, value: AuthorizationMethod) -> Self {
        self.authorization_method = Some(value);
        self
    }

    pub fn charging(mut self, value: ConversationChargingCommonModel) -> Self {
        self.charging = Some(value);
        self
    }

    pub fn phone_call(mut self, value: ConversationHistoryMetadataCommonModelPhoneCall) -> Self {
        self.phone_call = Some(value);
        self
    }

    pub fn batch_call(mut self, value: ConversationHistoryBatchCallModel) -> Self {
        self.batch_call = Some(value);
        self
    }

    pub fn termination_reason(mut self, value: impl Into<String>) -> Self {
        self.termination_reason = Some(value.into());
        self
    }

    pub fn error(mut self, value: ConversationHistoryErrorCommonModel) -> Self {
        self.error = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn main_language(mut self, value: impl Into<String>) -> Self {
        self.main_language = Some(value.into());
        self
    }

    pub fn rag_usage(mut self, value: ConversationHistoryRagUsageCommonModel) -> Self {
        self.rag_usage = Some(value);
        self
    }

    pub fn text_only(mut self, value: bool) -> Self {
        self.text_only = Some(value);
        self
    }

    pub fn features_usage(mut self, value: FeaturesUsageCommonModel) -> Self {
        self.features_usage = Some(value);
        self
    }

    pub fn eleven_assistant(mut self, value: ConversationHistoryElevenAssistantCommonModel) -> Self {
        self.eleven_assistant = Some(value);
        self
    }

    pub fn initiator_id(mut self, value: impl Into<String>) -> Self {
        self.initiator_id = Some(value.into());
        self
    }

    pub fn conversation_initiation_source(mut self, value: ConversationInitiationSource) -> Self {
        self.conversation_initiation_source = Some(value);
        self
    }

    pub fn conversation_initiation_source_version(mut self, value: impl Into<String>) -> Self {
        self.conversation_initiation_source_version = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn async_metadata(mut self, value: AsyncConversationMetadata) -> Self {
        self.async_metadata = Some(value);
        self
    }

    pub fn whatsapp(mut self, value: WhatsAppConversationInfo) -> Self {
        self.whatsapp = Some(value);
        self
    }

    pub fn sms(mut self, value: SmsConversationInfo) -> Self {
        self.sms = Some(value);
        self
    }

    pub fn agent_created_from(mut self, value: AgentDefinitionSource) -> Self {
        self.agent_created_from = Some(value);
        self
    }

    pub fn agent_last_updated_from(mut self, value: AgentDefinitionSource) -> Self {
        self.agent_last_updated_from = Some(value);
        self
    }

    pub fn voice_rewards(mut self, value: Vec<ConversationVoiceRewardModel>) -> Self {
        self.voice_rewards = Some(value);
        self
    }

    pub fn cost_fiat(mut self, value: f64) -> Self {
        self.cost_fiat = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryMetadataCommonModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time_unix_secs`](ConversationHistoryMetadataCommonModelBuilder::start_time_unix_secs)
    /// - [`call_duration_secs`](ConversationHistoryMetadataCommonModelBuilder::call_duration_secs)
    pub fn build(self) -> Result<ConversationHistoryMetadataCommonModel, BuildError> {
        Ok(ConversationHistoryMetadataCommonModel {
            start_time_unix_secs: self.start_time_unix_secs.ok_or_else(|| BuildError::missing_field("start_time_unix_secs"))?,
            accepted_time_unix_secs: self.accepted_time_unix_secs,
            call_duration_secs: self.call_duration_secs.ok_or_else(|| BuildError::missing_field("call_duration_secs"))?,
            cost: self.cost,
            deletion_settings: self.deletion_settings,
            feedback: self.feedback,
            authorization_method: self.authorization_method,
            charging: self.charging,
            phone_call: self.phone_call,
            batch_call: self.batch_call,
            termination_reason: self.termination_reason,
            error: self.error,
            warnings: self.warnings,
            main_language: self.main_language,
            rag_usage: self.rag_usage,
            text_only: self.text_only,
            features_usage: self.features_usage,
            eleven_assistant: self.eleven_assistant,
            initiator_id: self.initiator_id,
            conversation_initiation_source: self.conversation_initiation_source,
            conversation_initiation_source_version: self.conversation_initiation_source_version,
            timezone: self.timezone,
            async_metadata: self.async_metadata,
            whatsapp: self.whatsapp,
            sms: self.sms,
            agent_created_from: self.agent_created_from,
            agent_last_updated_from: self.agent_last_updated_from,
            voice_rewards: self.voice_rewards,
            cost_fiat: self.cost_fiat,
        })
    }
}
