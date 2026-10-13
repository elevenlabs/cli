pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListResponseAgentDeploymentHistoryItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ListResponseMeta>,
    #[serde(default)]
    pub results: Vec<AgentDeploymentHistoryItem>,
}

impl ListResponseAgentDeploymentHistoryItem {
    pub fn builder() -> ListResponseAgentDeploymentHistoryItemBuilder {
        <ListResponseAgentDeploymentHistoryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseAgentDeploymentHistoryItemBuilder {
    meta: Option<ListResponseMeta>,
    results: Option<Vec<AgentDeploymentHistoryItem>>,
}

impl ListResponseAgentDeploymentHistoryItemBuilder {
    pub fn meta(mut self, value: ListResponseMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<AgentDeploymentHistoryItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListResponseAgentDeploymentHistoryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](ListResponseAgentDeploymentHistoryItemBuilder::results)
    pub fn build(self) -> Result<ListResponseAgentDeploymentHistoryItem, BuildError> {
        Ok(ListResponseAgentDeploymentHistoryItem {
            meta: self.meta,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
