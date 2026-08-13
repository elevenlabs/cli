pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationInitiationClientDataRequestInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_config_override: Option<ConversationConfigClientOverrideInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_llm_extra_body: Option<HashMap<String, serde_json::Value>>,
    /// ID of the end user participating in this conversation (for agent owner's user identification)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_info: Option<ConversationInitiationSourceInfo>,
    /// ID of the agent branch to use for this conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Environment to use for resolving environment variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// If set, start the workflow at this node id instead of the default entry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_workflow_node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_variables: Option<HashMap<String, serde_json::Value>>,
}

impl ConversationInitiationClientDataRequestInput {
    pub fn builder() -> ConversationInitiationClientDataRequestInputBuilder {
        <ConversationInitiationClientDataRequestInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationInitiationClientDataRequestInputBuilder {
    conversation_config_override: Option<ConversationConfigClientOverrideInput>,
    custom_llm_extra_body: Option<HashMap<String, serde_json::Value>>,
    user_id: Option<String>,
    source_info: Option<ConversationInitiationSourceInfo>,
    branch_id: Option<String>,
    environment: Option<String>,
    starting_workflow_node_id: Option<String>,
    dynamic_variables: Option<HashMap<String, serde_json::Value>>,
}

impl ConversationInitiationClientDataRequestInputBuilder {
    pub fn conversation_config_override(mut self, value: ConversationConfigClientOverrideInput) -> Self {
        self.conversation_config_override = Some(value);
        self
    }

    pub fn custom_llm_extra_body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom_llm_extra_body = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn source_info(mut self, value: ConversationInitiationSourceInfo) -> Self {
        self.source_info = Some(value);
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn starting_workflow_node_id(mut self, value: impl Into<String>) -> Self {
        self.starting_workflow_node_id = Some(value.into());
        self
    }

    pub fn dynamic_variables(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.dynamic_variables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationInitiationClientDataRequestInput`].
    pub fn build(self) -> Result<ConversationInitiationClientDataRequestInput, BuildError> {
        Ok(ConversationInitiationClientDataRequestInput {
            conversation_config_override: self.conversation_config_override,
            custom_llm_extra_body: self.custom_llm_extra_body,
            user_id: self.user_id,
            source_info: self.source_info,
            branch_id: self.branch_id,
            environment: self.environment,
            starting_workflow_node_id: self.starting_workflow_node_id,
            dynamic_variables: self.dynamic_variables,
        })
    }
}
