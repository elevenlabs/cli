pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubagentRunResultDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl SubagentRunResultDetails {
    pub fn builder() -> SubagentRunResultDetailsBuilder {
        <SubagentRunResultDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubagentRunResultDetailsBuilder {
    sub_conversation_id: Option<String>,
    agent_id: Option<String>,
}

impl SubagentRunResultDetailsBuilder {
    pub fn sub_conversation_id(mut self, value: impl Into<String>) -> Self {
        self.sub_conversation_id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubagentRunResultDetails`].
    pub fn build(self) -> Result<SubagentRunResultDetails, BuildError> {
        Ok(SubagentRunResultDetails {
            sub_conversation_id: self.sub_conversation_id,
            agent_id: self.agent_id,
        })
    }
}
