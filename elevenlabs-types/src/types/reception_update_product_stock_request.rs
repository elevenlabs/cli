pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionUpdateProductStockRequest {
    /// Location whose count is set.
    #[serde(default)]
    pub location_id: String,
    /// The new on-hand quantity.
    #[serde(default)]
    pub on_hand: i64,
    /// On-hand value the caller last saw. The write is rejected with a 409 if it no longer matches, so a sale made while the caller was editing is never overwritten.
    #[serde(default)]
    pub expected_on_hand: i64,
}

impl ReceptionUpdateProductStockRequest {
    pub fn builder() -> ReceptionUpdateProductStockRequestBuilder {
        <ReceptionUpdateProductStockRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionUpdateProductStockRequestBuilder {
    location_id: Option<String>,
    on_hand: Option<i64>,
    expected_on_hand: Option<i64>,
}

impl ReceptionUpdateProductStockRequestBuilder {
    pub fn location_id(mut self, value: impl Into<String>) -> Self {
        self.location_id = Some(value.into());
        self
    }

    pub fn on_hand(mut self, value: i64) -> Self {
        self.on_hand = Some(value);
        self
    }

    pub fn expected_on_hand(mut self, value: i64) -> Self {
        self.expected_on_hand = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionUpdateProductStockRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`location_id`](ReceptionUpdateProductStockRequestBuilder::location_id)
    /// - [`on_hand`](ReceptionUpdateProductStockRequestBuilder::on_hand)
    /// - [`expected_on_hand`](ReceptionUpdateProductStockRequestBuilder::expected_on_hand)
    pub fn build(self) -> Result<ReceptionUpdateProductStockRequest, BuildError> {
        Ok(ReceptionUpdateProductStockRequest {
            location_id: self.location_id.ok_or_else(|| BuildError::missing_field("location_id"))?,
            on_hand: self.on_hand.ok_or_else(|| BuildError::missing_field("on_hand"))?,
            expected_on_hand: self.expected_on_hand.ok_or_else(|| BuildError::missing_field("expected_on_hand"))?,
        })
    }
}

