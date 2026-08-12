pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseToolInfo {
    /// Search strategies exposed to the model. Must be non-empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_strategies: Option<Vec<SearchStrategy>>,
}

impl KnowledgeBaseToolInfo {
    pub fn builder() -> KnowledgeBaseToolInfoBuilder {
        <KnowledgeBaseToolInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseToolInfoBuilder {
    enabled_strategies: Option<Vec<SearchStrategy>>,
}

impl KnowledgeBaseToolInfoBuilder {
    pub fn enabled_strategies(mut self, value: Vec<SearchStrategy>) -> Self {
        self.enabled_strategies = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseToolInfo`].
    pub fn build(self) -> Result<KnowledgeBaseToolInfo, BuildError> {
        Ok(KnowledgeBaseToolInfo {
            enabled_strategies: self.enabled_strategies,
        })
    }
}
