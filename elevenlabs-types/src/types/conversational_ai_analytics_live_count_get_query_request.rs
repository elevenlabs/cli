pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationalAiAnalyticsLiveCountGetQueryRequest {
    /// The id of an agent to restrict the analytics to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl ConversationalAiAnalyticsLiveCountGetQueryRequest {
    pub fn builder() -> ConversationalAiAnalyticsLiveCountGetQueryRequestBuilder {
        <ConversationalAiAnalyticsLiveCountGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationalAiAnalyticsLiveCountGetQueryRequestBuilder {
    agent_id: Option<String>,
}

impl ConversationalAiAnalyticsLiveCountGetQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationalAiAnalyticsLiveCountGetQueryRequest`].
    pub fn build(self) -> Result<ConversationalAiAnalyticsLiveCountGetQueryRequest, BuildError> {
        Ok(ConversationalAiAnalyticsLiveCountGetQueryRequest {
            agent_id: self.agent_id,
        })
    }
}

