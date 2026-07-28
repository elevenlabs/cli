pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseRagToolConfig {
}

impl KnowledgeBaseRagToolConfig {
    pub fn builder() -> KnowledgeBaseRagToolConfigBuilder {
        <KnowledgeBaseRagToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseRagToolConfigBuilder {
}

impl KnowledgeBaseRagToolConfigBuilder {

    /// Consumes the builder and constructs a [`KnowledgeBaseRagToolConfig`].
    pub fn build(self) -> Result<KnowledgeBaseRagToolConfig, BuildError> {
        Ok(KnowledgeBaseRagToolConfig {
        })
    }
}
