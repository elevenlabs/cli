pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CallingPeriodPolicy {
    #[serde(default)]
    pub weekdays: Vec<i64>,
    /// Windows on each of those days. They may not overlap on any day.
    #[serde(default)]
    pub intervals: Vec<CallingIntervalPolicy>,
}

impl CallingPeriodPolicy {
    pub fn builder() -> CallingPeriodPolicyBuilder {
        <CallingPeriodPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CallingPeriodPolicyBuilder {
    weekdays: Option<Vec<i64>>,
    intervals: Option<Vec<CallingIntervalPolicy>>,
}

impl CallingPeriodPolicyBuilder {
    pub fn weekdays(mut self, value: Vec<i64>) -> Self {
        self.weekdays = Some(value);
        self
    }

    pub fn intervals(mut self, value: Vec<CallingIntervalPolicy>) -> Self {
        self.intervals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CallingPeriodPolicy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`weekdays`](CallingPeriodPolicyBuilder::weekdays)
    /// - [`intervals`](CallingPeriodPolicyBuilder::intervals)
    pub fn build(self) -> Result<CallingPeriodPolicy, BuildError> {
        Ok(CallingPeriodPolicy {
            weekdays: self.weekdays.ok_or_else(|| BuildError::missing_field("weekdays"))?,
            intervals: self.intervals.ok_or_else(|| BuildError::missing_field("intervals"))?,
        })
    }
}
