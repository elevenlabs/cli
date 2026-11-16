pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionUpdateHolidayRequestPatch {
    /// Display name of the closure or time off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// First day of the closure or time off, inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateModel>,
    /// Last day of the closure or time off, inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateModel>,
    /// 'closed' for no availability, or 'override' to use override_hours instead of the regular hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holiday_type: Option<HolidayType>,
    /// Replacement working hours when holiday_type is 'override'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_hours: Option<Vec<TimeRangeModel>>,
    /// What the entry applies to: a location, a staff member, or an asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_off_type: Option<TimeOffType>,
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

impl ReceptionUpdateHolidayRequestPatch {
    pub fn builder() -> ReceptionUpdateHolidayRequestPatchBuilder {
        <ReceptionUpdateHolidayRequestPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionUpdateHolidayRequestPatchBuilder {
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

impl ReceptionUpdateHolidayRequestPatchBuilder {
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

    /// Consumes the builder and constructs a [`ReceptionUpdateHolidayRequestPatch`].
    pub fn build(self) -> Result<ReceptionUpdateHolidayRequestPatch, BuildError> {
        Ok(ReceptionUpdateHolidayRequestPatch {
            name: self.name,
            start_date: self.start_date,
            end_date: self.end_date,
            holiday_type: self.holiday_type,
            override_hours: self.override_hours,
            time_off_type: self.time_off_type,
            resource_id: self.resource_id,
            location_id: self.location_id,
            reason: self.reason,
        })
    }
}

