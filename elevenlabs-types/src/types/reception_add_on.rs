pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A standalone add-on that can be tacked onto a service or rental booking.
/// 
/// An add-on is a small, free-form extra (e.g. "Hot towel", "Valet parking")
/// that extends the parent booking's duration and total price without
/// creating a separate calendar event.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionAddOn {
    /// Stable identifier used by booking requests and calendar snapshots. Survives name/price edits, so rebooking and reporting stay consistent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Add-on name shown to customers and the receptionist.
    #[serde(default)]
    pub name: StrippedStr,
    /// Short owner-authored blurb shown to customers and voice agents when they're choosing add-ons. Empty string means no description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<StrippedStr>,
    /// Minutes added to the parent booking when this add-on is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_duration_minutes: Option<i64>,
    /// Amount added to the parent booking's total when this add-on is selected.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub add_price: Option<f64>,
}

impl ReceptionAddOn {
    pub fn builder() -> ReceptionAddOnBuilder {
        <ReceptionAddOnBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionAddOnBuilder {
    id: Option<String>,
    name: Option<StrippedStr>,
    description: Option<StrippedStr>,
    add_duration_minutes: Option<i64>,
    add_price: Option<f64>,
}

impl ReceptionAddOnBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: StrippedStr) -> Self {
        self.name = Some(value);
        self
    }

    pub fn description(mut self, value: StrippedStr) -> Self {
        self.description = Some(value);
        self
    }

    pub fn add_duration_minutes(mut self, value: i64) -> Self {
        self.add_duration_minutes = Some(value);
        self
    }

    pub fn add_price(mut self, value: f64) -> Self {
        self.add_price = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionAddOn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionAddOnBuilder::name)
    pub fn build(self) -> Result<ReceptionAddOn, BuildError> {
        Ok(ReceptionAddOn {
            id: self.id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            add_duration_minutes: self.add_duration_minutes,
            add_price: self.add_price,
        })
    }
}
