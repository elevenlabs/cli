pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionHolidaysListQueryRequest {
    /// Filter entries overlapping from this timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_unix: Option<i64>,
    /// Filter entries overlapping before this timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_unix: Option<i64>,
    /// Filter by type: location, staff, or asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_off_type: Option<TimeOffType>,
    /// Filter by associated resource ID (staff/asset)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Filter by location ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
}

impl ReceptionHolidaysListQueryRequest {
    pub fn builder() -> ReceptionHolidaysListQueryRequestBuilder {
        <ReceptionHolidaysListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionHolidaysListQueryRequestBuilder {
    start_unix: Option<i64>,
    end_unix: Option<i64>,
    time_off_type: Option<TimeOffType>,
    resource_id: Option<String>,
    location_id: Option<String>,
}

impl ReceptionHolidaysListQueryRequestBuilder {
    pub fn start_unix(mut self, value: i64) -> Self {
        self.start_unix = Some(value);
        self
    }

    pub fn end_unix(mut self, value: i64) -> Self {
        self.end_unix = Some(value);
        self
    }

    pub fn time_off_type(mut self, value: TimeOffType) -> Self {
        self.time_off_type = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn location_id(mut self, value: impl Into<String>) -> Self {
        self.location_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceptionHolidaysListQueryRequest`].
    pub fn build(self) -> Result<ReceptionHolidaysListQueryRequest, BuildError> {
        Ok(ReceptionHolidaysListQueryRequest {
            start_unix: self.start_unix,
            end_unix: self.end_unix,
            time_off_type: self.time_off_type,
            resource_id: self.resource_id,
            location_id: self.location_id,
        })
    }
}

