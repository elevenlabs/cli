pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RagDocumentIndexesResponseModel {
    #[serde(default)]
    pub indexes: Vec<RagDocumentIndexResponseModel>,
}

impl RagDocumentIndexesResponseModel {
    pub fn builder() -> RagDocumentIndexesResponseModelBuilder {
        <RagDocumentIndexesResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RagDocumentIndexesResponseModelBuilder {
    indexes: Option<Vec<RagDocumentIndexResponseModel>>,
}

impl RagDocumentIndexesResponseModelBuilder {
    pub fn indexes(mut self, value: Vec<RagDocumentIndexResponseModel>) -> Self {
        self.indexes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RagDocumentIndexesResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`indexes`](RagDocumentIndexesResponseModelBuilder::indexes)
    pub fn build(self) -> Result<RagDocumentIndexesResponseModel, BuildError> {
        Ok(RagDocumentIndexesResponseModel {
            indexes: self.indexes.ok_or_else(|| BuildError::missing_field("indexes"))?,
        })
    }
}
