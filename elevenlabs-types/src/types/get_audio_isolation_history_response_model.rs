pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetAudioIsolationHistoryResponseModel {
    #[serde(default)]
    pub items: Vec<AudioIsolationHistoryItemResponseModel>,
    #[serde(default)]
    pub has_more: bool,
}

impl GetAudioIsolationHistoryResponseModel {
    pub fn builder() -> GetAudioIsolationHistoryResponseModelBuilder {
        <GetAudioIsolationHistoryResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAudioIsolationHistoryResponseModelBuilder {
    items: Option<Vec<AudioIsolationHistoryItemResponseModel>>,
    has_more: Option<bool>,
}

impl GetAudioIsolationHistoryResponseModelBuilder {
    pub fn items(mut self, value: Vec<AudioIsolationHistoryItemResponseModel>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn has_more(mut self, value: bool) -> Self {
        self.has_more = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAudioIsolationHistoryResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](GetAudioIsolationHistoryResponseModelBuilder::items)
    /// - [`has_more`](GetAudioIsolationHistoryResponseModelBuilder::has_more)
    pub fn build(self) -> Result<GetAudioIsolationHistoryResponseModel, BuildError> {
        Ok(GetAudioIsolationHistoryResponseModel {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            has_more: self.has_more.ok_or_else(|| BuildError::missing_field("has_more"))?,
        })
    }
}
