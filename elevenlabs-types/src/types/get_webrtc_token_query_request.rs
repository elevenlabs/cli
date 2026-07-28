pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_webrtc_token
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetWebrtcTokenQueryRequest {
    /// Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    #[serde(default)]
    pub agent_id: String,
    /// Optional custom participant name. If not provided, user ID will be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_name: Option<String>,
    /// The ID of the branch to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// The environment to use for resolving environment variables (e.g. 'production', 'staging'). Defaults to 'production'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl GetWebrtcTokenQueryRequest {
    pub fn builder() -> GetWebrtcTokenQueryRequestBuilder {
        <GetWebrtcTokenQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWebrtcTokenQueryRequestBuilder {
    agent_id: Option<String>,
    participant_name: Option<String>,
    branch_id: Option<String>,
    environment: Option<String>,
}

impl GetWebrtcTokenQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn participant_name(mut self, value: impl Into<String>) -> Self {
        self.participant_name = Some(value.into());
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

    /// Consumes the builder and constructs a [`GetWebrtcTokenQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](GetWebrtcTokenQueryRequestBuilder::agent_id)
    pub fn build(self) -> Result<GetWebrtcTokenQueryRequest, BuildError> {
        Ok(GetWebrtcTokenQueryRequest {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            participant_name: self.participant_name,
            branch_id: self.branch_id,
            environment: self.environment,
        })
    }
}

