pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResellerUpcomingPaymentsResponse {
    #[serde(default)]
    pub payments: Vec<ResellerUpcomingPayment>,
}

impl ResellerUpcomingPaymentsResponse {
    pub fn builder() -> ResellerUpcomingPaymentsResponseBuilder {
        <ResellerUpcomingPaymentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResellerUpcomingPaymentsResponseBuilder {
    payments: Option<Vec<ResellerUpcomingPayment>>,
}

impl ResellerUpcomingPaymentsResponseBuilder {
    pub fn payments(mut self, value: Vec<ResellerUpcomingPayment>) -> Self {
        self.payments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResellerUpcomingPaymentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payments`](ResellerUpcomingPaymentsResponseBuilder::payments)
    pub fn build(self) -> Result<ResellerUpcomingPaymentsResponse, BuildError> {
        Ok(ResellerUpcomingPaymentsResponse {
            payments: self.payments.ok_or_else(|| BuildError::missing_field("payments"))?,
        })
    }
}
