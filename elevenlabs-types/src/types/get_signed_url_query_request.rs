pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_signed_url
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSignedUrlQueryRequest {
    /// Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    #[serde(default)]
    pub agent_id: String,
    /// Whether to include a conversation_id with the response. If included, the conversation_signature cannot be used again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_conversation_id: Option<bool>,
    /// The ID of the branch to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// The environment to use for resolving environment variables (e.g. 'production', 'staging'). Defaults to 'production'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Whether to enable debug events. Only available for users with editor access to the agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_events_request: Option<bool>,
}

impl GetSignedUrlQueryRequest {
    pub fn builder() -> GetSignedUrlQueryRequestBuilder {
        <GetSignedUrlQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSignedUrlQueryRequestBuilder {
    agent_id: Option<String>,
    include_conversation_id: Option<bool>,
    branch_id: Option<String>,
    environment: Option<String>,
    debug_events_request: Option<bool>,
}

impl GetSignedUrlQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn include_conversation_id(mut self, value: bool) -> Self {
        self.include_conversation_id = Some(value);
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

    pub fn debug_events_request(mut self, value: bool) -> Self {
        self.debug_events_request = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSignedUrlQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](GetSignedUrlQueryRequestBuilder::agent_id)
    pub fn build(self) -> Result<GetSignedUrlQueryRequest, BuildError> {
        Ok(GetSignedUrlQueryRequest {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            include_conversation_id: self.include_conversation_id,
            branch_id: self.branch_id,
            environment: self.environment,
            debug_events_request: self.debug_events_request,
        })
    }
}

