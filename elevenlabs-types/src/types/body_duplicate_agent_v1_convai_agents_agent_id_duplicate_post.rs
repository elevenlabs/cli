pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePost {
    /// A name to make the agent easier to find
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePost {
    pub fn builder() -> BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePostBuilder {
        <BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePostBuilder {
    name: Option<String>,
}

impl BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePostBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePost`].
    pub fn build(self) -> Result<BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePost, BuildError> {
        Ok(BodyDuplicateAgentV1ConvaiAgentsAgentIdDuplicatePost {
            name: self.name,
        })
    }
}

