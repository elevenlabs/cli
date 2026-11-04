pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnalysisItemsResponseModel {
    #[serde(default)]
    pub analysis_items: Vec<AnalysisItemResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl AnalysisItemsResponseModel {
    pub fn builder() -> AnalysisItemsResponseModelBuilder {
        <AnalysisItemsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnalysisItemsResponseModelBuilder {
    analysis_items: Option<Vec<AnalysisItemResponseModel>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl AnalysisItemsResponseModelBuilder {
    pub fn analysis_items(mut self, value: Vec<AnalysisItemResponseModel>) -> Self {
        self.analysis_items = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AnalysisItemsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`analysis_items`](AnalysisItemsResponseModelBuilder::analysis_items)
    /// - [`has_more`](AnalysisItemsResponseModelBuilder::has_more)
    pub fn build(self) -> Result<AnalysisItemsResponseModel, BuildError> {
        Ok(AnalysisItemsResponseModel {
            analysis_items: self.analysis_items.ok_or_else(|| BuildError::missing_field("analysis_items"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
