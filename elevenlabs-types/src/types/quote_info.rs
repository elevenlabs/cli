pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QuoteInfo {
    /// The quoted price for this item in USD. Use the order's total_amount_usd for the combined order total.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount_usd: f64,
}

impl QuoteInfo {
    pub fn builder() -> QuoteInfoBuilder {
        <QuoteInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuoteInfoBuilder {
    amount_usd: Option<f64>,
}

impl QuoteInfoBuilder {
    pub fn amount_usd(mut self, value: f64) -> Self {
        self.amount_usd = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QuoteInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_usd`](QuoteInfoBuilder::amount_usd)
    pub fn build(self) -> Result<QuoteInfo, BuildError> {
        Ok(QuoteInfo {
            amount_usd: self.amount_usd.ok_or_else(|| BuildError::missing_field("amount_usd"))?,
        })
    }
}
