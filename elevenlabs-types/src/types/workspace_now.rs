pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceNow {
    /// Current date in the business's timezone.
    #[serde(default)]
    pub today: DateModel,
    /// Current hour (0-23)
    #[serde(default)]
    pub hour: i64,
    /// Current minute (0-59)
    #[serde(default)]
    pub minute: i64,
    /// Unix timestamp in milliseconds when this was generated
    #[serde(default)]
    pub fetched_at_ms: i64,
}

impl WorkspaceNow {
    pub fn builder() -> WorkspaceNowBuilder {
        <WorkspaceNowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceNowBuilder {
    today: Option<DateModel>,
    hour: Option<i64>,
    minute: Option<i64>,
    fetched_at_ms: Option<i64>,
}

impl WorkspaceNowBuilder {
    pub fn today(mut self, value: DateModel) -> Self {
        self.today = Some(value);
        self
    }

    pub fn hour(mut self, value: i64) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn minute(mut self, value: i64) -> Self {
        self.minute = Some(value);
        self
    }

    pub fn fetched_at_ms(mut self, value: i64) -> Self {
        self.fetched_at_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceNow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`today`](WorkspaceNowBuilder::today)
    /// - [`hour`](WorkspaceNowBuilder::hour)
    /// - [`minute`](WorkspaceNowBuilder::minute)
    /// - [`fetched_at_ms`](WorkspaceNowBuilder::fetched_at_ms)
    pub fn build(self) -> Result<WorkspaceNow, BuildError> {
        Ok(WorkspaceNow {
            today: self.today.ok_or_else(|| BuildError::missing_field("today"))?,
            hour: self.hour.ok_or_else(|| BuildError::missing_field("hour"))?,
            minute: self.minute.ok_or_else(|| BuildError::missing_field("minute"))?,
            fetched_at_ms: self.fetched_at_ms.ok_or_else(|| BuildError::missing_field("fetched_at_ms"))?,
        })
    }
}
