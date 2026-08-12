pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimeRange {
    #[serde(default)]
    pub start_ms: i64,
    #[serde(default)]
    pub end_ms: i64,
}

impl TimeRange {
    pub fn builder() -> TimeRangeBuilder {
        <TimeRangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimeRangeBuilder {
    start_ms: Option<i64>,
    end_ms: Option<i64>,
}

impl TimeRangeBuilder {
    pub fn start_ms(mut self, value: i64) -> Self {
        self.start_ms = Some(value);
        self
    }

    pub fn end_ms(mut self, value: i64) -> Self {
        self.end_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimeRange`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_ms`](TimeRangeBuilder::start_ms)
    /// - [`end_ms`](TimeRangeBuilder::end_ms)
    pub fn build(self) -> Result<TimeRange, BuildError> {
        Ok(TimeRange {
            start_ms: self.start_ms.ok_or_else(|| BuildError::missing_field("start_ms"))?,
            end_ms: self.end_ms.ok_or_else(|| BuildError::missing_field("end_ms"))?,
        })
    }
}
