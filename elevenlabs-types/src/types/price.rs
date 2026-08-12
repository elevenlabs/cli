pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Currency/amount pair.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Price {
    #[serde(default)]
    pub amount: String,
    pub currency: Currency,
}

impl Price {
    pub fn builder() -> PriceBuilder {
        <PriceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PriceBuilder {
    amount: Option<String>,
    currency: Option<Currency>,
}

impl PriceBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn currency(mut self, value: Currency) -> Self {
        self.currency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Price`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PriceBuilder::amount)
    /// - [`currency`](PriceBuilder::currency)
    pub fn build(self) -> Result<Price, BuildError> {
        Ok(Price {
            amount: self.amount.ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self.currency.ok_or_else(|| BuildError::missing_field("currency"))?,
        })
    }
}
