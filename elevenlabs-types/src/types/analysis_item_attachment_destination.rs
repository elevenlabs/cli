pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnalysisItemAttachmentDestination {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub branch_id: String,
}

impl AnalysisItemAttachmentDestination {
    pub fn builder() -> AnalysisItemAttachmentDestinationBuilder {
        <AnalysisItemAttachmentDestinationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisItemAttachmentDestinationBuilder {
    agent_id: Option<String>,
    branch_id: Option<String>,
}

impl AnalysisItemAttachmentDestinationBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnalysisItemAttachmentDestination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](AnalysisItemAttachmentDestinationBuilder::agent_id)
    /// - [`branch_id`](AnalysisItemAttachmentDestinationBuilder::branch_id)
    pub fn build(self) -> Result<AnalysisItemAttachmentDestination, BuildError> {
        Ok(AnalysisItemAttachmentDestination {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            branch_id: self.branch_id.ok_or_else(|| BuildError::missing_field("branch_id"))?,
        })
    }
}
