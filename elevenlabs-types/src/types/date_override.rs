pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DateOverride {
    /// First local date the override applies on.
    #[serde(default)]
    pub start_date: String,
    /// Last local date the override applies on, inclusive.
    #[serde(default)]
    pub end_date: String,
    /// Calling windows on every day from `start_date` to `end_date`, replacing the weekly ones. Empty means no calls on those days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intervals: Option<Vec<CallingIntervalPolicy>>,
    /// What the override is for, such as `Christmas`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DateOverride {
    pub fn builder() -> DateOverrideBuilder {
        <DateOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DateOverrideBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    intervals: Option<Vec<CallingIntervalPolicy>>,
    name: Option<String>,
}

impl DateOverrideBuilder {
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn intervals(mut self, value: Vec<CallingIntervalPolicy>) -> Self {
        self.intervals = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DateOverride`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_date`](DateOverrideBuilder::start_date)
    /// - [`end_date`](DateOverrideBuilder::end_date)
    pub fn build(self) -> Result<DateOverride, BuildError> {
        Ok(DateOverride {
            start_date: self.start_date.ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date.ok_or_else(|| BuildError::missing_field("end_date"))?,
            intervals: self.intervals,
            name: self.name,
        })
    }
}
