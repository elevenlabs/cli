pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CallingPeriod {
    /// Days the intervals apply on, from `0` (Monday) to `6` (Sunday).
    #[serde(default)]
    pub weekdays: Vec<i64>,
    /// Windows on each of those days. They may not overlap on any day.
    #[serde(default)]
    pub intervals: Vec<CallingIntervalPolicy>,
}

impl CallingPeriod {
    pub fn builder() -> CallingPeriodBuilder {
        <CallingPeriodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CallingPeriodBuilder {
    weekdays: Option<Vec<i64>>,
    intervals: Option<Vec<CallingIntervalPolicy>>,
}

impl CallingPeriodBuilder {
    pub fn weekdays(mut self, value: Vec<i64>) -> Self {
        self.weekdays = Some(value);
        self
    }

    pub fn intervals(mut self, value: Vec<CallingIntervalPolicy>) -> Self {
        self.intervals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CallingPeriod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`weekdays`](CallingPeriodBuilder::weekdays)
    /// - [`intervals`](CallingPeriodBuilder::intervals)
    pub fn build(self) -> Result<CallingPeriod, BuildError> {
        Ok(CallingPeriod {
            weekdays: self.weekdays.ok_or_else(|| BuildError::missing_field("weekdays"))?,
            intervals: self.intervals.ok_or_else(|| BuildError::missing_field("intervals"))?,
        })
    }
}
