pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReceptionCreateHolidayRequest {
    /// Display name of the closure or time off.
    #[serde(default)]
    pub name: String,
    /// First day of the closure or time off, inclusive.
    #[serde(default)]
    pub start_date: DateModel,
    /// Last day of the closure or time off, inclusive.
    #[serde(default)]
    pub end_date: DateModel,
    /// 'closed' for no availability, or 'override' to use override_hours instead of the regular hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holiday_type: Option<HolidayType>,
    /// Replacement working hours when holiday_type is 'override'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_hours: Option<Vec<TimeRangeModel>>,
    /// What the entry applies to: a location, a staff member, or an asset.
    pub time_off_type: TimeOffType,
    /// The staff member or asset ID; required when time_off_type is 'staff' or 'asset'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// For LOCATION type: the location this closure applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    /// Optional note on why the closure or time off happens, up to 500 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl ReceptionCreateHolidayRequest {
    pub fn builder() -> ReceptionCreateHolidayRequestBuilder {
        <ReceptionCreateHolidayRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionCreateHolidayRequestBuilder {
    name: Option<String>,
    start_date: Option<DateModel>,
    end_date: Option<DateModel>,
    holiday_type: Option<HolidayType>,
    override_hours: Option<Vec<TimeRangeModel>>,
    time_off_type: Option<TimeOffType>,
    resource_id: Option<String>,
    location_id: Option<String>,
    reason: Option<String>,
}

impl ReceptionCreateHolidayRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: DateModel) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateModel) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn holiday_type(mut self, value: HolidayType) -> Self {
        self.holiday_type = Some(value);
        self
    }

    pub fn override_hours(mut self, value: Vec<TimeRangeModel>) -> Self {
        self.override_hours = Some(value);
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

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceptionCreateHolidayRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionCreateHolidayRequestBuilder::name)
    /// - [`start_date`](ReceptionCreateHolidayRequestBuilder::start_date)
    /// - [`end_date`](ReceptionCreateHolidayRequestBuilder::end_date)
    /// - [`time_off_type`](ReceptionCreateHolidayRequestBuilder::time_off_type)
    pub fn build(self) -> Result<ReceptionCreateHolidayRequest, BuildError> {
        Ok(ReceptionCreateHolidayRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            start_date: self.start_date.ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self.end_date.ok_or_else(|| BuildError::missing_field("end_date"))?,
            holiday_type: self.holiday_type,
            override_hours: self.override_hours,
            time_off_type: self.time_off_type.ok_or_else(|| BuildError::missing_field("time_off_type"))?,
            resource_id: self.resource_id,
            location_id: self.location_id,
            reason: self.reason,
        })
    }
}

