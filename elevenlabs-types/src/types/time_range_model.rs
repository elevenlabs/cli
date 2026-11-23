pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Represents a time-of-day range (e.g., 09:00-17:00) commonly used for
/// working hours, available hours, or recurring schedules.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimeRangeModel {
    /// Start hour (0-23)
    #[serde(default)]
    pub start_hour: i64,
    /// Start minute (0-59)
    #[serde(default)]
    pub start_minute: i64,
    /// End hour (0-24, where 24:00 means midnight end-of-day)
    #[serde(default)]
    pub end_hour: i64,
    /// End minute (0-59)
    #[serde(default)]
    pub end_minute: i64,
}

impl TimeRangeModel {
    pub fn builder() -> TimeRangeModelBuilder {
        <TimeRangeModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeRangeModelBuilder {
    start_hour: Option<i64>,
    start_minute: Option<i64>,
    end_hour: Option<i64>,
    end_minute: Option<i64>,
}

impl TimeRangeModelBuilder {
    pub fn start_hour(mut self, value: i64) -> Self {
        self.start_hour = Some(value);
        self
    }

    pub fn start_minute(mut self, value: i64) -> Self {
        self.start_minute = Some(value);
        self
    }

    pub fn end_hour(mut self, value: i64) -> Self {
        self.end_hour = Some(value);
        self
    }

    pub fn end_minute(mut self, value: i64) -> Self {
        self.end_minute = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimeRangeModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_hour`](TimeRangeModelBuilder::start_hour)
    /// - [`start_minute`](TimeRangeModelBuilder::start_minute)
    /// - [`end_hour`](TimeRangeModelBuilder::end_hour)
    /// - [`end_minute`](TimeRangeModelBuilder::end_minute)
    pub fn build(self) -> Result<TimeRangeModel, BuildError> {
        Ok(TimeRangeModel {
            start_hour: self.start_hour.ok_or_else(|| BuildError::missing_field("start_hour"))?,
            start_minute: self.start_minute.ok_or_else(|| BuildError::missing_field("start_minute"))?,
            end_hour: self.end_hour.ok_or_else(|| BuildError::missing_field("end_hour"))?,
            end_minute: self.end_minute.ok_or_else(|| BuildError::missing_field("end_minute"))?,
        })
    }
}
