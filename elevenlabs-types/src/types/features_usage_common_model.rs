pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeaturesUsageCommonModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_detection: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_to_agent: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_to_number: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multivoice: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtmf_tones: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_mcp_servers: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_zrm_workspace: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_zrm_agent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_dynamic_variable_updates: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_livekit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voicemail_detection: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtmf_input: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<WorkflowFeaturesUsageCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_testing: Option<TestsFeatureUsageCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning: Option<FeatureStatusCommonModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_input: Option<FeatureStatusCommonModel>,
}

impl FeaturesUsageCommonModel {
    pub fn builder() -> FeaturesUsageCommonModelBuilder {
        <FeaturesUsageCommonModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeaturesUsageCommonModelBuilder {
    language_detection: Option<FeatureStatusCommonModel>,
    transfer_to_agent: Option<FeatureStatusCommonModel>,
    transfer_to_number: Option<FeatureStatusCommonModel>,
    multivoice: Option<FeatureStatusCommonModel>,
    dtmf_tones: Option<FeatureStatusCommonModel>,
    external_mcp_servers: Option<FeatureStatusCommonModel>,
    pii_zrm_workspace: Option<bool>,
    pii_zrm_agent: Option<bool>,
    tool_dynamic_variable_updates: Option<FeatureStatusCommonModel>,
    is_livekit: Option<bool>,
    voicemail_detection: Option<FeatureStatusCommonModel>,
    dtmf_input: Option<FeatureStatusCommonModel>,
    workflow: Option<WorkflowFeaturesUsageCommonModel>,
    agent_testing: Option<TestsFeatureUsageCommonModel>,
    versioning: Option<FeatureStatusCommonModel>,
    file_input: Option<FeatureStatusCommonModel>,
}

impl FeaturesUsageCommonModelBuilder {
    pub fn language_detection(mut self, value: FeatureStatusCommonModel) -> Self {
        self.language_detection = Some(value);
        self
    }

    pub fn transfer_to_agent(mut self, value: FeatureStatusCommonModel) -> Self {
        self.transfer_to_agent = Some(value);
        self
    }

    pub fn transfer_to_number(mut self, value: FeatureStatusCommonModel) -> Self {
        self.transfer_to_number = Some(value);
        self
    }

    pub fn multivoice(mut self, value: FeatureStatusCommonModel) -> Self {
        self.multivoice = Some(value);
        self
    }

    pub fn dtmf_tones(mut self, value: FeatureStatusCommonModel) -> Self {
        self.dtmf_tones = Some(value);
        self
    }

    pub fn external_mcp_servers(mut self, value: FeatureStatusCommonModel) -> Self {
        self.external_mcp_servers = Some(value);
        self
    }

    pub fn pii_zrm_workspace(mut self, value: bool) -> Self {
        self.pii_zrm_workspace = Some(value);
        self
    }

    pub fn pii_zrm_agent(mut self, value: bool) -> Self {
        self.pii_zrm_agent = Some(value);
        self
    }

    pub fn tool_dynamic_variable_updates(mut self, value: FeatureStatusCommonModel) -> Self {
        self.tool_dynamic_variable_updates = Some(value);
        self
    }

    pub fn is_livekit(mut self, value: bool) -> Self {
        self.is_livekit = Some(value);
        self
    }

    pub fn voicemail_detection(mut self, value: FeatureStatusCommonModel) -> Self {
        self.voicemail_detection = Some(value);
        self
    }

    pub fn dtmf_input(mut self, value: FeatureStatusCommonModel) -> Self {
        self.dtmf_input = Some(value);
        self
    }

    pub fn workflow(mut self, value: WorkflowFeaturesUsageCommonModel) -> Self {
        self.workflow = Some(value);
        self
    }

    pub fn agent_testing(mut self, value: TestsFeatureUsageCommonModel) -> Self {
        self.agent_testing = Some(value);
        self
    }

    pub fn versioning(mut self, value: FeatureStatusCommonModel) -> Self {
        self.versioning = Some(value);
        self
    }

    pub fn file_input(mut self, value: FeatureStatusCommonModel) -> Self {
        self.file_input = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeaturesUsageCommonModel`].
    pub fn build(self) -> Result<FeaturesUsageCommonModel, BuildError> {
        Ok(FeaturesUsageCommonModel {
            language_detection: self.language_detection,
            transfer_to_agent: self.transfer_to_agent,
            transfer_to_number: self.transfer_to_number,
            multivoice: self.multivoice,
            dtmf_tones: self.dtmf_tones,
            external_mcp_servers: self.external_mcp_servers,
            pii_zrm_workspace: self.pii_zrm_workspace,
            pii_zrm_agent: self.pii_zrm_agent,
            tool_dynamic_variable_updates: self.tool_dynamic_variable_updates,
            is_livekit: self.is_livekit,
            voicemail_detection: self.voicemail_detection,
            dtmf_input: self.dtmf_input,
            workflow: self.workflow,
            agent_testing: self.agent_testing,
            versioning: self.versioning,
            file_input: self.file_input,
        })
    }
}
