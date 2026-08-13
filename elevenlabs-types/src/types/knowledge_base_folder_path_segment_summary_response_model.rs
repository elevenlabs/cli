pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseFolderPathSegmentSummaryResponseModel {
    #[serde(default)]
    pub id: String,
}

impl KnowledgeBaseFolderPathSegmentSummaryResponseModel {
    pub fn builder() -> KnowledgeBaseFolderPathSegmentSummaryResponseModelBuilder {
        <KnowledgeBaseFolderPathSegmentSummaryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseFolderPathSegmentSummaryResponseModelBuilder {
    id: Option<String>,
}

impl KnowledgeBaseFolderPathSegmentSummaryResponseModelBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseFolderPathSegmentSummaryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](KnowledgeBaseFolderPathSegmentSummaryResponseModelBuilder::id)
    pub fn build(self) -> Result<KnowledgeBaseFolderPathSegmentSummaryResponseModel, BuildError> {
        Ok(KnowledgeBaseFolderPathSegmentSummaryResponseModel {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
