pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentPlatformSettingsResponseModel {
    /// Settings for evaluation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<EvaluationSettingsOutput>,
    /// Configuration for the widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget: Option<WidgetConfig>,
    /// Data collection settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection: Option<HashMap<String, AnalysisProperty>>,
    /// Scope per data collection item ID. Missing keys default to conversation scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_collection_scopes: Option<HashMap<String, AnalysisScope>>,
    /// Additional overrides for the agent during conversation initiation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<ConversationInitiationClientDataConfigOutput>,
    /// Workspace overrides for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_overrides: Option<AgentWorkspaceOverridesOutput>,
    /// Testing configuration for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testing: Option<AgentTestingSettings>,
    /// Whether the agent is archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Guardrails configuration for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<GuardrailsV1Output>,
    /// Language for all conversation analysis outputs (summaries, titles, evaluation rationales, data collection rationales). If not set, the language will be inferred from the conversation. Must be one of the supported conversation languages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_language: Option<String>,
    /// When enabled, a conversation transcript is automatically translated to the viewer's application language when they open the transcript page. If not set or false, transcripts are shown in their original language unless the viewer manually selects a translation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_translate_transcript_to_app_language: Option<bool>,
    /// Settings for authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthSettings>,
    /// Call limits for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_limits: Option<AgentCallLimits>,
    /// Privacy settings for the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<PrivacyConfigOutput>,
    /// The trust context in which the agent operates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_context: Option<AgentTrustContext>,
    /// Default LLM model for post-call analysis (evaluation and data collection)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_llm: Option<Llm>,
    /// Per-agent topic discovery configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_discovery: Option<TopicDiscoverySettings>,
    /// Per-agent post-call sentiment analysis configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_analysis: Option<SentimentAnalysisSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety: Option<SafetyResponseModel>,
}

impl AgentPlatformSettingsResponseModel {
    pub fn builder() -> AgentPlatformSettingsResponseModelBuilder {
        <AgentPlatformSettingsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentPlatformSettingsResponseModelBuilder {
    evaluation: Option<EvaluationSettingsOutput>,
    widget: Option<WidgetConfig>,
    data_collection: Option<HashMap<String, AnalysisProperty>>,
    data_collection_scopes: Option<HashMap<String, AnalysisScope>>,
    overrides: Option<ConversationInitiationClientDataConfigOutput>,
    workspace_overrides: Option<AgentWorkspaceOverridesOutput>,
    testing: Option<AgentTestingSettings>,
    archived: Option<bool>,
    guardrails: Option<GuardrailsV1Output>,
    summary_language: Option<String>,
    auto_translate_transcript_to_app_language: Option<bool>,
    auth: Option<AuthSettings>,
    call_limits: Option<AgentCallLimits>,
    privacy: Option<PrivacyConfigOutput>,
    trust_context: Option<AgentTrustContext>,
    analysis_llm: Option<Llm>,
    topic_discovery: Option<TopicDiscoverySettings>,
    sentiment_analysis: Option<SentimentAnalysisSettings>,
    safety: Option<SafetyResponseModel>,
}

impl AgentPlatformSettingsResponseModelBuilder {
    pub fn evaluation(mut self, value: EvaluationSettingsOutput) -> Self {
        self.evaluation = Some(value);
        self
    }

    pub fn widget(mut self, value: WidgetConfig) -> Self {
        self.widget = Some(value);
        self
    }

    pub fn data_collection(mut self, value: HashMap<String, AnalysisProperty>) -> Self {
        self.data_collection = Some(value);
        self
    }

    pub fn data_collection_scopes(mut self, value: HashMap<String, AnalysisScope>) -> Self {
        self.data_collection_scopes = Some(value);
        self
    }

    pub fn overrides(mut self, value: ConversationInitiationClientDataConfigOutput) -> Self {
        self.overrides = Some(value);
        self
    }

    pub fn workspace_overrides(mut self, value: AgentWorkspaceOverridesOutput) -> Self {
        self.workspace_overrides = Some(value);
        self
    }

    pub fn testing(mut self, value: AgentTestingSettings) -> Self {
        self.testing = Some(value);
        self
    }

    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn guardrails(mut self, value: GuardrailsV1Output) -> Self {
        self.guardrails = Some(value);
        self
    }

    pub fn summary_language(mut self, value: impl Into<String>) -> Self {
        self.summary_language = Some(value.into());
        self
    }

    pub fn auto_translate_transcript_to_app_language(mut self, value: bool) -> Self {
        self.auto_translate_transcript_to_app_language = Some(value);
        self
    }

    pub fn auth(mut self, value: AuthSettings) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn call_limits(mut self, value: AgentCallLimits) -> Self {
        self.call_limits = Some(value);
        self
    }

    pub fn privacy(mut self, value: PrivacyConfigOutput) -> Self {
        self.privacy = Some(value);
        self
    }

    pub fn trust_context(mut self, value: AgentTrustContext) -> Self {
        self.trust_context = Some(value);
        self
    }

    pub fn analysis_llm(mut self, value: Llm) -> Self {
        self.analysis_llm = Some(value);
        self
    }

    pub fn topic_discovery(mut self, value: TopicDiscoverySettings) -> Self {
        self.topic_discovery = Some(value);
        self
    }

    pub fn sentiment_analysis(mut self, value: SentimentAnalysisSettings) -> Self {
        self.sentiment_analysis = Some(value);
        self
    }

    pub fn safety(mut self, value: SafetyResponseModel) -> Self {
        self.safety = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentPlatformSettingsResponseModel`].
    pub fn build(self) -> Result<AgentPlatformSettingsResponseModel, BuildError> {
        Ok(AgentPlatformSettingsResponseModel {
            evaluation: self.evaluation,
            widget: self.widget,
            data_collection: self.data_collection,
            data_collection_scopes: self.data_collection_scopes,
            overrides: self.overrides,
            workspace_overrides: self.workspace_overrides,
            testing: self.testing,
            archived: self.archived,
            guardrails: self.guardrails,
            summary_language: self.summary_language,
            auto_translate_transcript_to_app_language: self.auto_translate_transcript_to_app_language,
            auth: self.auth,
            call_limits: self.call_limits,
            privacy: self.privacy,
            trust_context: self.trust_context,
            analysis_llm: self.analysis_llm,
            topic_discovery: self.topic_discovery,
            sentiment_analysis: self.sentiment_analysis,
            safety: self.safety,
        })
    }
}
