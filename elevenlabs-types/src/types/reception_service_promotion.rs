pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A time-bound promotion on a service/variant or on add-ons.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionServicePromotion {
    /// Stable ID of this promotion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Display name of the promotion.
    #[serde(default)]
    pub name: StrippedStr,
    /// Whether this promotion targets the service/variant price or add-on prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<PromotionTarget>,
    /// When set (target=service), the promotion applies only to this variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
    /// When set (target=add_on), the promotion applies only to this add-on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_on_id: Option<String>,
    /// The final price when this promotion is applied to a matching line item.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub promotional_price: f64,
    /// Days when the promotion is active (0=Monday, 6=Sunday); empty means every day.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_of_week: Option<Vec<i64>>,
    /// First day the promotion applies, inclusive.
    #[serde(default)]
    pub start_date: String,
    /// Last day the promotion applies, inclusive; on or after `start_date`.
    #[serde(default)]
    pub end_date: String,
    /// Whether the promotion applies; inactive promotions are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl ReceptionServicePromotion {
    pub fn builder() -> ReceptionServicePromotionBuilder {
        <ReceptionServicePromotionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionServicePromotionBuilder {
    id: Option<String>,
    name: Option<StrippedStr>,
    target: Option<PromotionTarget>,
    variant_id: Option<String>,
    add_on_id: Option<String>,
    promotional_price: Option<f64>,
    days_of_week: Option<Vec<i64>>,
    start_date: Option<String>,
    end_date: Option<String>,
    is_active: Option<bool>,
}

impl ReceptionServicePromotionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: StrippedStr) -> Self {
        self.name = Some(value);
        self
    }

    pub fn target(mut self, value: PromotionTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    pub fn add_on_id(mut self, value: impl Into<String>) -> Self {
        self.add_on_id = Some(value.into());
        self
    }

    pub fn promotional_price(mut self, value: f64) -> Self {
        self.promotional_price = Some(value);
        self
    }

    pub fn days_of_week(mut self, value: Vec<i64>) -> Self {
        self.days_of_week = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionServicePromotion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionServicePromotionBuilder::name)
    /// - [`promotional_price`](ReceptionServicePromotionBuilder::promotional_price)
    /// - [`start_date`](ReceptionServicePromotionBuilder::start_date)
    /// - [`end_date`](ReceptionServicePromotionBuilder::end_date)
    pub fn build(self) -> Result<ReceptionServicePromotion, BuildError> {
        Ok(ReceptionServicePromotion {
            id: self.id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            target: self.target,
            variant_id: self.variant_id,
            add_on_id: self.add_on_id,
            promotional_price: self.promotional_price.ok_or_else(|| BuildError::missing_field("promotional_price"))?,
            days_of_week: self.days_of_week,
            start_date: self.start_date.ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date.ok_or_else(|| BuildError::missing_field("end_date"))?,
            is_active: self.is_active,
        })
    }
}
