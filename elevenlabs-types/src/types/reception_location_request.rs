pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionLocationRequest {
    /// Internal name, e.g. 'Main Office' or 'Downtown Branch', shown in the dashboard and owner notifications.
    #[serde(default)]
    pub name: StrippedStr,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<StrippedStr>,
    /// Directions or other notes for customers, e.g. 'Free parking behind the building'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<StrippedStr>,
    /// Per-location operating hours. On create, omit to have the server prefill from the most recently added active location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_hours: Option<WorkingHours>,
    /// Name shown to customers; empty reuses name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_facing_name: Option<StrippedStr>,
    /// Maximum number of appointments that may overlap at this location at a single instant. Only enforced while the booking page setting concurrent_limit_enabled is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_appointments: Option<i64>,
    /// Whether the location is active. Inactive locations take no bookings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl ReceptionLocationRequest {
    pub fn builder() -> ReceptionLocationRequestBuilder {
        <ReceptionLocationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionLocationRequestBuilder {
    name: Option<StrippedStr>,
    address: Option<StrippedStr>,
    notes: Option<StrippedStr>,
    operating_hours: Option<WorkingHours>,
    customer_facing_name: Option<StrippedStr>,
    max_concurrent_appointments: Option<i64>,
    is_active: Option<bool>,
}

impl ReceptionLocationRequestBuilder {
    pub fn name(mut self, value: StrippedStr) -> Self {
        self.name = Some(value);
        self
    }

    pub fn address(mut self, value: StrippedStr) -> Self {
        self.address = Some(value);
        self
    }

    pub fn notes(mut self, value: StrippedStr) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn operating_hours(mut self, value: WorkingHours) -> Self {
        self.operating_hours = Some(value);
        self
    }

    pub fn customer_facing_name(mut self, value: StrippedStr) -> Self {
        self.customer_facing_name = Some(value);
        self
    }

    pub fn max_concurrent_appointments(mut self, value: i64) -> Self {
        self.max_concurrent_appointments = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionLocationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionLocationRequestBuilder::name)
    pub fn build(self) -> Result<ReceptionLocationRequest, BuildError> {
        Ok(ReceptionLocationRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            address: self.address,
            notes: self.notes,
            operating_hours: self.operating_hours,
            customer_facing_name: self.customer_facing_name,
            max_concurrent_appointments: self.max_concurrent_appointments,
            is_active: self.is_active,
        })
    }
}

