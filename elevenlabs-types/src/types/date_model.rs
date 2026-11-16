pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Represents a calendar date as individual numeric components.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DateModel {
    /// Year
    #[serde(default)]
    pub year: i64,
    /// Month (1-12)
    #[serde(default)]
    pub month: i64,
    /// Day of month (1-31)
    #[serde(default)]
    pub day: i64,
}

impl DateModel {
    pub fn builder() -> DateModelBuilder {
        <DateModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DateModelBuilder {
    year: Option<i64>,
    month: Option<i64>,
    day: Option<i64>,
}

impl DateModelBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn day(mut self, value: i64) -> Self {
        self.day = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DateModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](DateModelBuilder::year)
    /// - [`month`](DateModelBuilder::month)
    /// - [`day`](DateModelBuilder::day)
    pub fn build(self) -> Result<DateModel, BuildError> {
        Ok(DateModel {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self.month.ok_or_else(|| BuildError::missing_field("month"))?,
            day: self.day.ok_or_else(|| BuildError::missing_field("day"))?,
        })
    }
}
