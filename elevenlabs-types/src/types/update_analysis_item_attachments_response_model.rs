pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAnalysisItemAttachmentsResponseModel {
    #[serde(default)]
    pub results: Vec<UpdateAnalysisItemAttachmentsResponseModelResultsItem>,
}

impl UpdateAnalysisItemAttachmentsResponseModel {
    pub fn builder() -> UpdateAnalysisItemAttachmentsResponseModelBuilder {
        <UpdateAnalysisItemAttachmentsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAnalysisItemAttachmentsResponseModelBuilder {
    results: Option<Vec<UpdateAnalysisItemAttachmentsResponseModelResultsItem>>,
}

impl UpdateAnalysisItemAttachmentsResponseModelBuilder {
    pub fn results(mut self, value: Vec<UpdateAnalysisItemAttachmentsResponseModelResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAnalysisItemAttachmentsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](UpdateAnalysisItemAttachmentsResponseModelBuilder::results)
    pub fn build(self) -> Result<UpdateAnalysisItemAttachmentsResponseModel, BuildError> {
        Ok(UpdateAnalysisItemAttachmentsResponseModel {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
