pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DubbingMetadataPageResponseModel {
    #[serde(default)]
    pub dubs: Vec<DubbingMetadataResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

impl DubbingMetadataPageResponseModel {
    pub fn builder() -> DubbingMetadataPageResponseModelBuilder {
        <DubbingMetadataPageResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingMetadataPageResponseModelBuilder {
    dubs: Option<Vec<DubbingMetadataResponse>>,
    next_cursor: Option<String>,
    has_more: Option<bool>,
}

impl DubbingMetadataPageResponseModelBuilder {
    pub fn dubs(mut self, value: Vec<DubbingMetadataResponse>) -> Self {
        self.dubs = Some(value);
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

    /// Consumes the builder and constructs a [`DubbingMetadataPageResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dubs`](DubbingMetadataPageResponseModelBuilder::dubs)
    /// - [`has_more`](DubbingMetadataPageResponseModelBuilder::has_more)
    pub fn build(self) -> Result<DubbingMetadataPageResponseModel, BuildError> {
        Ok(DubbingMetadataPageResponseModel {
            dubs: self.dubs.ok_or_else(|| BuildError::missing_field("dubs"))?,
            next_cursor: self.next_cursor,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
