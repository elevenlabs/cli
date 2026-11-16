pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionAdjustProductStockRequest {
    /// Location whose count changes.
    #[serde(default)]
    pub location_id: String,
    /// Signed change to apply to on-hand. Prefer this over an absolute write whenever the intent is 'a few more/fewer'.
    #[serde(default)]
    pub delta: i64,
}

impl ReceptionAdjustProductStockRequest {
    pub fn builder() -> ReceptionAdjustProductStockRequestBuilder {
        <ReceptionAdjustProductStockRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionAdjustProductStockRequestBuilder {
    location_id: Option<String>,
    delta: Option<i64>,
}

impl ReceptionAdjustProductStockRequestBuilder {
    pub fn location_id(mut self, value: impl Into<String>) -> Self {
        self.location_id = Some(value.into());
        self
    }

    pub fn delta(mut self, value: i64) -> Self {
        self.delta = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionAdjustProductStockRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location_id`](ReceptionAdjustProductStockRequestBuilder::location_id)
    /// - [`delta`](ReceptionAdjustProductStockRequestBuilder::delta)
    pub fn build(self) -> Result<ReceptionAdjustProductStockRequest, BuildError> {
        Ok(ReceptionAdjustProductStockRequest {
            location_id: self.location_id.ok_or_else(|| BuildError::missing_field("location_id"))?,
            delta: self.delta.ok_or_else(|| BuildError::missing_field("delta"))?,
        })
    }
}

