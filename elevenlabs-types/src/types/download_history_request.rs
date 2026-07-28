pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DownloadHistoryRequest {
    /// A list of history items to download, you can get IDs of history items and other metadata using the GET https://api.elevenlabs.io/v1/history endpoint.
    #[serde(default)]
    pub history_item_ids: Vec<String>,
    /// Output format to transcode the audio file, can be wav or default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

impl DownloadHistoryRequest {
    pub fn builder() -> DownloadHistoryRequestBuilder {
        <DownloadHistoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DownloadHistoryRequestBuilder {
    history_item_ids: Option<Vec<String>>,
    output_format: Option<String>,
}

impl DownloadHistoryRequestBuilder {
    pub fn history_item_ids(mut self, value: Vec<String>) -> Self {
        self.history_item_ids = Some(value);
        self
    }

    pub fn output_format(mut self, value: impl Into<String>) -> Self {
        self.output_format = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DownloadHistoryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`history_item_ids`](DownloadHistoryRequestBuilder::history_item_ids)
    pub fn build(self) -> Result<DownloadHistoryRequest, BuildError> {
        Ok(DownloadHistoryRequest {
            history_item_ids: self.history_item_ids.ok_or_else(|| BuildError::missing_field("history_item_ids"))?,
            output_format: self.output_format,
        })
    }
}

