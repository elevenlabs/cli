pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetAgentKnowledgebaseSizeResponseModel {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub number_of_pages: f64,
}

impl GetAgentKnowledgebaseSizeResponseModel {
    pub fn builder() -> GetAgentKnowledgebaseSizeResponseModelBuilder {
        <GetAgentKnowledgebaseSizeResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAgentKnowledgebaseSizeResponseModelBuilder {
    number_of_pages: Option<f64>,
}

impl GetAgentKnowledgebaseSizeResponseModelBuilder {
    pub fn number_of_pages(mut self, value: f64) -> Self {
        self.number_of_pages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAgentKnowledgebaseSizeResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`number_of_pages`](GetAgentKnowledgebaseSizeResponseModelBuilder::number_of_pages)
    pub fn build(self) -> Result<GetAgentKnowledgebaseSizeResponseModel, BuildError> {
        Ok(GetAgentKnowledgebaseSizeResponseModel {
            number_of_pages: self.number_of_pages.ok_or_else(|| BuildError::missing_field("number_of_pages"))?,
        })
    }
}
