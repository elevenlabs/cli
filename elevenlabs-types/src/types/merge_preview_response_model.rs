pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MergePreviewResponseModel {
    /// The ID of the agent
    #[serde(default)]
    pub agent_id: String,
    /// The name of the agent
    #[serde(default)]
    pub name: String,
    /// The conversation configuration of the agent
    #[serde(default)]
    pub conversation_config: ConversationalConfig,
    /// The metadata of the agent
    #[serde(default)]
    pub metadata: AgentMetadataResponseModel,
    /// The platform settings of the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_settings: Option<AgentPlatformSettingsResponseModel>,
    /// The phone numbers of the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<MergePreviewResponseModelPhoneNumbersItem>>,
    /// WhatsApp accounts assigned to the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whatsapp_accounts: Option<Vec<GetWhatsAppAccountResponse>>,
    /// The workflow of the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow: Option<AgentWorkflowResponseModel>,
    /// The access information of the agent for the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_info: Option<ResourceAccessInfo>,
    /// Agent tags used to categorize the agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The ID of the version the agent is on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// The ID of the branch the agent is on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// The ID of the main branch for this agent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_branch_id: Option<String>,
    /// Dot-paths of config fields where both branches modified the same field relative to their common ancestor (conflicts). Present regardless of which side wins the conflict.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridden_fields: Option<Vec<String>>,
    /// True when the merge/rebase would be a no-op, i.e. the merged result is identical to the source branch tip. The rebase endpoint rejects in this case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identical_to_target: Option<bool>,
}

impl MergePreviewResponseModel {
    pub fn builder() -> MergePreviewResponseModelBuilder {
        <MergePreviewResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergePreviewResponseModelBuilder {
    agent_id: Option<String>,
    name: Option<String>,
    conversation_config: Option<ConversationalConfig>,
    metadata: Option<AgentMetadataResponseModel>,
    platform_settings: Option<AgentPlatformSettingsResponseModel>,
    phone_numbers: Option<Vec<MergePreviewResponseModelPhoneNumbersItem>>,
    whatsapp_accounts: Option<Vec<GetWhatsAppAccountResponse>>,
    workflow: Option<AgentWorkflowResponseModel>,
    access_info: Option<ResourceAccessInfo>,
    tags: Option<Vec<String>>,
    version_id: Option<String>,
    branch_id: Option<String>,
    main_branch_id: Option<String>,
    overridden_fields: Option<Vec<String>>,
    source_identical_to_target: Option<bool>,
}

impl MergePreviewResponseModelBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn conversation_config(mut self, value: ConversationalConfig) -> Self {
        self.conversation_config = Some(value);
        self
    }

    pub fn metadata(mut self, value: AgentMetadataResponseModel) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn platform_settings(mut self, value: AgentPlatformSettingsResponseModel) -> Self {
        self.platform_settings = Some(value);
        self
    }

    pub fn phone_numbers(mut self, value: Vec<MergePreviewResponseModelPhoneNumbersItem>) -> Self {
        self.phone_numbers = Some(value);
        self
    }

    pub fn whatsapp_accounts(mut self, value: Vec<GetWhatsAppAccountResponse>) -> Self {
        self.whatsapp_accounts = Some(value);
        self
    }

    pub fn workflow(mut self, value: AgentWorkflowResponseModel) -> Self {
        self.workflow = Some(value);
        self
    }

    pub fn access_info(mut self, value: ResourceAccessInfo) -> Self {
        self.access_info = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn main_branch_id(mut self, value: impl Into<String>) -> Self {
        self.main_branch_id = Some(value.into());
        self
    }

    pub fn overridden_fields(mut self, value: Vec<String>) -> Self {
        self.overridden_fields = Some(value);
        self
    }

    pub fn source_identical_to_target(mut self, value: bool) -> Self {
        self.source_identical_to_target = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergePreviewResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](MergePreviewResponseModelBuilder::agent_id)
    /// - [`name`](MergePreviewResponseModelBuilder::name)
    /// - [`conversation_config`](MergePreviewResponseModelBuilder::conversation_config)
    /// - [`metadata`](MergePreviewResponseModelBuilder::metadata)
    pub fn build(self) -> Result<MergePreviewResponseModel, BuildError> {
        Ok(MergePreviewResponseModel {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            conversation_config: self.conversation_config.ok_or_else(|| BuildError::missing_field("conversation_config"))?,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            platform_settings: self.platform_settings,
            phone_numbers: self.phone_numbers,
            whatsapp_accounts: self.whatsapp_accounts,
            workflow: self.workflow,
            access_info: self.access_info,
            tags: self.tags,
            version_id: self.version_id,
            branch_id: self.branch_id,
            main_branch_id: self.main_branch_id,
            overridden_fields: self.overridden_fields,
            source_identical_to_target: self.source_identical_to_target,
        })
    }
}
