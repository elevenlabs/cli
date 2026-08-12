pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseDeletedResponseModel {
    #[serde(default)]
    pub id: String,
}

impl KnowledgeBaseDeletedResponseModel {
    pub fn builder() -> KnowledgeBaseDeletedResponseModelBuilder {
        <KnowledgeBaseDeletedResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseDeletedResponseModelBuilder {
    id: Option<String>,
}

impl KnowledgeBaseDeletedResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseDeletedResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](KnowledgeBaseDeletedResponseModelBuilder::id)
    pub fn build(self) -> Result<KnowledgeBaseDeletedResponseModel, BuildError> {
        Ok(KnowledgeBaseDeletedResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
