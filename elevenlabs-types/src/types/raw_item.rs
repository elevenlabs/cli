pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single item extracted from a page — could be a service, person, product, or asset.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RawItem {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i64>,
}

impl RawItem {
    pub fn builder() -> RawItemBuilder {
        <RawItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RawItemBuilder {
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    duration_minutes: Option<i64>,
}

impl RawItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn duration_minutes(mut self, value: i64) -> Self {
        self.duration_minutes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RawItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RawItemBuilder::name)
    pub fn build(self) -> Result<RawItem, BuildError> {
        Ok(RawItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            price: self.price,
            duration_minutes: self.duration_minutes,
        })
    }
}
