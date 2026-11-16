pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single business location stored inline in agent info.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionLocationInput {
    /// Internal name, e.g. 'Main Office' or 'Downtown Branch', shown in the dashboard and owner notifications.
    #[serde(default)]
    pub name: StrippedStr,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<StrippedStr>,
    /// Directions or other notes for customers, e.g. 'Free parking behind the building'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<StrippedStr>,
    /// When this location is open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_hours: Option<WorkingHours>,
    /// Name shown to customers; empty reuses name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_facing_name: Option<StrippedStr>,
    /// Maximum number of appointments that may overlap at this location at a single instant. Only enforced while the booking page setting concurrent_limit_enabled is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_appointments: Option<i64>,
    #[serde(default)]
    pub id: String,
    /// Unix timestamp when this location was deactivated. None when active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated_at_unix: Option<i64>,
    /// Why this location was deactivated. None when active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivation_reason: Option<ReceptionLocationDeactivationReason>,
}

impl ReceptionLocationInput {
    pub fn builder() -> ReceptionLocationInputBuilder {
        <ReceptionLocationInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionLocationInputBuilder {
    name: Option<StrippedStr>,
    address: Option<StrippedStr>,
    notes: Option<StrippedStr>,
    operating_hours: Option<WorkingHours>,
    customer_facing_name: Option<StrippedStr>,
    max_concurrent_appointments: Option<i64>,
    id: Option<String>,
    deactivated_at_unix: Option<i64>,
    deactivation_reason: Option<ReceptionLocationDeactivationReason>,
}

impl ReceptionLocationInputBuilder {
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn deactivated_at_unix(mut self, value: i64) -> Self {
        self.deactivated_at_unix = Some(value);
        self
    }

    pub fn deactivation_reason(mut self, value: ReceptionLocationDeactivationReason) -> Self {
        self.deactivation_reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionLocationInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionLocationInputBuilder::name)
    /// - [`id`](ReceptionLocationInputBuilder::id)
    pub fn build(self) -> Result<ReceptionLocationInput, BuildError> {
        Ok(ReceptionLocationInput {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            address: self.address,
            notes: self.notes,
            operating_hours: self.operating_hours,
            customer_facing_name: self.customer_facing_name,
            max_concurrent_appointments: self.max_concurrent_appointments,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            deactivated_at_unix: self.deactivated_at_unix,
            deactivation_reason: self.deactivation_reason,
        })
    }
}
