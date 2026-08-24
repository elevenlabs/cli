pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrderDeliverablesResponse {
    /// The list of delivered files for the order. Empty if the order is not yet completed.
    #[serde(default)]
    pub deliverables: Vec<DeliverableInfo>,
}

impl OrderDeliverablesResponse {
    pub fn builder() -> OrderDeliverablesResponseBuilder {
        <OrderDeliverablesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrderDeliverablesResponseBuilder {
    deliverables: Option<Vec<DeliverableInfo>>,
}

impl OrderDeliverablesResponseBuilder {
    pub fn deliverables(mut self, value: Vec<DeliverableInfo>) -> Self {
        self.deliverables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrderDeliverablesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deliverables`](OrderDeliverablesResponseBuilder::deliverables)
    pub fn build(self) -> Result<OrderDeliverablesResponse, BuildError> {
        Ok(OrderDeliverablesResponse {
            deliverables: self.deliverables.ok_or_else(|| BuildError::missing_field("deliverables"))?,
        })
    }
}
