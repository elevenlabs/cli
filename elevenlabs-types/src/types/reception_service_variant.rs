pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A variant of a service identified by its duration, with optional price override.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionServiceVariant {
    /// Stable ID of this option; keep it when editing so bookings and promotions still match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// How long this option takes, in minutes.
    #[serde(default)]
    pub duration_minutes: i64,
    /// Price of this option in the workspace's currency; null means no listed price.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    /// Whether customers can book this option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl ReceptionServiceVariant {
    pub fn builder() -> ReceptionServiceVariantBuilder {
        <ReceptionServiceVariantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionServiceVariantBuilder {
    id: Option<String>,
    duration_minutes: Option<i64>,
    price: Option<f64>,
    is_active: Option<bool>,
}

impl ReceptionServiceVariantBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn duration_minutes(mut self, value: i64) -> Self {
        self.duration_minutes = Some(value);
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionServiceVariant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`duration_minutes`](ReceptionServiceVariantBuilder::duration_minutes)
    pub fn build(self) -> Result<ReceptionServiceVariant, BuildError> {
        Ok(ReceptionServiceVariant {
            id: self.id,
            duration_minutes: self.duration_minutes.ok_or_else(|| BuildError::missing_field("duration_minutes"))?,
            price: self.price,
            is_active: self.is_active,
        })
    }
}
