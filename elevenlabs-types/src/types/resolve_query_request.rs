pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for resolve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolveQueryRequest {
    /// Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource.
    #[serde(default)]
    pub agent_id: String,
    /// A Slack message URL or a Zendesk ticket URL.
    #[serde(default)]
    pub reference: String,
}

impl ResolveQueryRequest {
    pub fn builder() -> ResolveQueryRequestBuilder {
        <ResolveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolveQueryRequestBuilder {
    agent_id: Option<String>,
    reference: Option<String>,
}

impl ResolveQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResolveQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](ResolveQueryRequestBuilder::agent_id)
    /// - [`reference`](ResolveQueryRequestBuilder::reference)
    pub fn build(self) -> Result<ResolveQueryRequest, BuildError> {
        Ok(ResolveQueryRequest {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            reference: self.reference.ok_or_else(|| BuildError::missing_field("reference"))?,
        })
    }
}

