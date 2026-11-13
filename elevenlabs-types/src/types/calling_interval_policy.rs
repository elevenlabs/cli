pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CallingIntervalPolicy {
    /// Local start time, as `HH:MM`.
    #[serde(default)]
    pub start: String,
    /// Local end time, as `HH:MM`; `24:00` ends at midnight.
    #[serde(default)]
    pub end: String,
}

impl CallingIntervalPolicy {
    pub fn builder() -> CallingIntervalPolicyBuilder {
        <CallingIntervalPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CallingIntervalPolicyBuilder {
    start: Option<String>,
    end: Option<String>,
}

impl CallingIntervalPolicyBuilder {
    pub fn start(mut self, value: impl Into<String>) -> Self {
        self.start = Some(value.into());
        self
    }

    pub fn end(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CallingIntervalPolicy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](CallingIntervalPolicyBuilder::start)
    /// - [`end`](CallingIntervalPolicyBuilder::end)
    pub fn build(self) -> Result<CallingIntervalPolicy, BuildError> {
        Ok(CallingIntervalPolicy {
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
        })
    }
}
