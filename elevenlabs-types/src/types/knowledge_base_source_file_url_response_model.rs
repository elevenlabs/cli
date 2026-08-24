pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseSourceFileUrlResponseModel {
    /// Signed URL to download the source file directly
    #[serde(default)]
    pub signed_url: String,
}

impl KnowledgeBaseSourceFileUrlResponseModel {
    pub fn builder() -> KnowledgeBaseSourceFileUrlResponseModelBuilder {
        <KnowledgeBaseSourceFileUrlResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseSourceFileUrlResponseModelBuilder {
    signed_url: Option<String>,
}

impl KnowledgeBaseSourceFileUrlResponseModelBuilder {
    pub fn signed_url(mut self, value: impl Into<String>) -> Self {
        self.signed_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseSourceFileUrlResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`signed_url`](KnowledgeBaseSourceFileUrlResponseModelBuilder::signed_url)
    pub fn build(self) -> Result<KnowledgeBaseSourceFileUrlResponseModel, BuildError> {
        Ok(KnowledgeBaseSourceFileUrlResponseModel {
            signed_url: self.signed_url.ok_or_else(|| BuildError::missing_field("signed_url"))?,
        })
    }
}
