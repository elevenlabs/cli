pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HistoryItemResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_category: Option<serde_json::Value>,
}

impl HistoryItemResponse {
    pub fn builder() -> HistoryItemResponseBuilder {
        <HistoryItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HistoryItemResponseBuilder {
    state: Option<serde_json::Value>,
    voice_category: Option<serde_json::Value>,
}

impl HistoryItemResponseBuilder {
    pub fn state(mut self, value: serde_json::Value) -> Self {
        self.state = Some(value);
        self
    }

    pub fn voice_category(mut self, value: serde_json::Value) -> Self {
        self.voice_category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HistoryItemResponse`].
    pub fn build(self) -> Result<HistoryItemResponse, BuildError> {
        Ok(HistoryItemResponse {
            state: self.state,
            voice_category: self.voice_category,
        })
    }
}
