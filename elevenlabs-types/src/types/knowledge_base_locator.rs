pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseLocator {
    /// The type of the knowledge base
    pub r#type: KnowledgeBaseDocumentType,
    /// The name of the knowledge base
    #[serde(default)]
    pub name: String,
    /// The ID of the knowledge base
    #[serde(default)]
    pub id: String,
    /// The usage mode of the knowledge base
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_mode: Option<DocumentUsageModeEnum>,
}

impl KnowledgeBaseLocator {
    pub fn builder() -> KnowledgeBaseLocatorBuilder {
        <KnowledgeBaseLocatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseLocatorBuilder {
    r#type: Option<KnowledgeBaseDocumentType>,
    name: Option<String>,
    id: Option<String>,
    usage_mode: Option<DocumentUsageModeEnum>,
}

impl KnowledgeBaseLocatorBuilder {
    pub fn r#type(mut self, value: KnowledgeBaseDocumentType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn usage_mode(mut self, value: DocumentUsageModeEnum) -> Self {
        self.usage_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseLocator`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](KnowledgeBaseLocatorBuilder::r#type)
    /// - [`name`](KnowledgeBaseLocatorBuilder::name)
    /// - [`id`](KnowledgeBaseLocatorBuilder::id)
    pub fn build(self) -> Result<KnowledgeBaseLocator, BuildError> {
        Ok(KnowledgeBaseLocator {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            usage_mode: self.usage_mode,
        })
    }
}
