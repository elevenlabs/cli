pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentKnowledgeBaseRagListingResponseModel {
    /// Entries the agent would be shown, folders first and then by name. Large folders are cut to the number of entries the agent is shown.
    #[serde(default)]
    pub entries: Vec<AgentKnowledgeBaseRagListingEntryResponseModel>,
    /// Number of entries in the listed folder, including any not returned.
    #[serde(default)]
    pub total_count: i64,
}

impl AgentKnowledgeBaseRagListingResponseModel {
    pub fn builder() -> AgentKnowledgeBaseRagListingResponseModelBuilder {
        <AgentKnowledgeBaseRagListingResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentKnowledgeBaseRagListingResponseModelBuilder {
    entries: Option<Vec<AgentKnowledgeBaseRagListingEntryResponseModel>>,
    total_count: Option<i64>,
}

impl AgentKnowledgeBaseRagListingResponseModelBuilder {
    pub fn entries(mut self, value: Vec<AgentKnowledgeBaseRagListingEntryResponseModel>) -> Self {
        self.entries = Some(value);
        self
    }

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentKnowledgeBaseRagListingResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entries`](AgentKnowledgeBaseRagListingResponseModelBuilder::entries)
    /// - [`total_count`](AgentKnowledgeBaseRagListingResponseModelBuilder::total_count)
    pub fn build(self) -> Result<AgentKnowledgeBaseRagListingResponseModel, BuildError> {
        Ok(AgentKnowledgeBaseRagListingResponseModel {
            entries: self.entries.ok_or_else(|| BuildError::missing_field("entries"))?,
            total_count: self.total_count.ok_or_else(|| BuildError::missing_field("total_count"))?,
        })
    }
}
