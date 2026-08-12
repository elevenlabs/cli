pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseToolConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_strategies: Option<Vec<SearchStrategy>>,
}

impl KnowledgeBaseToolConfig {
    pub fn builder() -> KnowledgeBaseToolConfigBuilder {
        <KnowledgeBaseToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseToolConfigBuilder {
    enabled_strategies: Option<Vec<SearchStrategy>>,
}

impl KnowledgeBaseToolConfigBuilder {
    pub fn enabled_strategies(mut self, value: Vec<SearchStrategy>) -> Self {
        self.enabled_strategies = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseToolConfig`].
    pub fn build(self) -> Result<KnowledgeBaseToolConfig, BuildError> {
        Ok(KnowledgeBaseToolConfig {
            enabled_strategies: self.enabled_strategies,
        })
    }
}
