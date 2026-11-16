pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionServiceUpdateResponse {
    /// The updated service, or null when the update was held back because it reprices existing bookings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<ReceptionService>,
    /// True when the update changes the price of upcoming bookings and was not saved; send it again to the confirm-update endpoint to apply it.
    #[serde(default)]
    pub has_affected_events: bool,
}

impl ReceptionServiceUpdateResponse {
    pub fn builder() -> ReceptionServiceUpdateResponseBuilder {
        <ReceptionServiceUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionServiceUpdateResponseBuilder {
    service: Option<ReceptionService>,
    has_affected_events: Option<bool>,
}

impl ReceptionServiceUpdateResponseBuilder {
    pub fn service(mut self, value: ReceptionService) -> Self {
        self.service = Some(value);
        self
    }

    pub fn has_affected_events(mut self, value: bool) -> Self {
        self.has_affected_events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionServiceUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_affected_events`](ReceptionServiceUpdateResponseBuilder::has_affected_events)
    pub fn build(self) -> Result<ReceptionServiceUpdateResponse, BuildError> {
        Ok(ReceptionServiceUpdateResponse {
            service: self.service,
            has_affected_events: self.has_affected_events.ok_or_else(|| BuildError::missing_field("has_affected_events"))?,
        })
    }
}
