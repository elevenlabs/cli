pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkingHours {
    /// Open time ranges per day of the week, keyed 0 (Monday) to 6 (Sunday). A day with no ranges is closed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<HashMap<String, Vec<TimeRangeModel>>>,
    /// Timezone of these hours. Null uses the business's timezone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl WorkingHours {
    pub fn builder() -> WorkingHoursBuilder {
        <WorkingHoursBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkingHoursBuilder {
    hours: Option<HashMap<String, Vec<TimeRangeModel>>>,
    timezone: Option<String>,
}

impl WorkingHoursBuilder {
    pub fn hours(mut self, value: HashMap<String, Vec<TimeRangeModel>>) -> Self {
        self.hours = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkingHours`].
    pub fn build(self) -> Result<WorkingHours, BuildError> {
        Ok(WorkingHours {
            hours: self.hours,
            timezone: self.timezone,
        })
    }
}
