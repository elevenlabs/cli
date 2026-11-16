pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Represents a staff member who can provide services.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionStaffResource {
    /// Group memberships for this item. Each entry carries the group label and the item's 0-based rank within that group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ReceptionGroupMembership>>,
    /// Display position among items with no group; `null` sorts last. Change it with the reorder endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ungrouped_rank: Option<i64>,
    /// Internal/admin name of the staff member, shown in the dashboard and owner notifications.
    #[serde(default)]
    pub name: StrippedStr,
    /// Description or bio of the staff member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<StrippedStr>,
    /// Whether the staff member can be booked. Inactive staff keep their booking history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Email address of the staff member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Phone number of the staff member. Without a country code, +1 (US) is assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<PhoneNumberOptional>,
    /// Whether this staff member currently receives SMS booking notifications. Defaults to opted out — a staff member must be explicitly opted in; flipped to False automatically when they reply STOP to the workspace number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_notifications_opted_in: Option<bool>,
    /// URL of a photo of the staff member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Location this staff member is assigned to; empty means available at all locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    /// Custom working hours for this staff member. Null means follow the assigned location's operating_hours (requires location_id).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_hours: Option<WorkingHours>,
    /// Whether customers can see and pick this staff member by name on the booking page. False keeps them bookable, just not shown by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_on_booking_page: Option<bool>,
    /// Optional public-facing label. When empty, `name` is shown to customers too. `name` stays the internal/admin label used in the dashboard and business-owner notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_facing_name: Option<StrippedStr>,
    /// The staff member's ID.
    #[serde(default)]
    pub id: String,
    /// When this was created, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<UnixTimestamp>,
    /// When this was last updated, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_unix: Option<UnixTimestamp>,
}

impl ReceptionStaffResource {
    pub fn builder() -> ReceptionStaffResourceBuilder {
        <ReceptionStaffResourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionStaffResourceBuilder {
    groups: Option<Vec<ReceptionGroupMembership>>,
    ungrouped_rank: Option<i64>,
    name: Option<StrippedStr>,
    description: Option<StrippedStr>,
    is_active: Option<bool>,
    email: Option<String>,
    phone: Option<PhoneNumberOptional>,
    sms_notifications_opted_in: Option<bool>,
    photo_url: Option<String>,
    location_id: Option<String>,
    working_hours: Option<WorkingHours>,
    display_on_booking_page: Option<bool>,
    customer_facing_name: Option<StrippedStr>,
    id: Option<String>,
    created_at_unix: Option<UnixTimestamp>,
    updated_at_unix: Option<UnixTimestamp>,
}

impl ReceptionStaffResourceBuilder {
    pub fn groups(mut self, value: Vec<ReceptionGroupMembership>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn ungrouped_rank(mut self, value: i64) -> Self {
        self.ungrouped_rank = Some(value);
        self
    }

    pub fn name(mut self, value: StrippedStr) -> Self {
        self.name = Some(value);
        self
    }

    pub fn description(mut self, value: StrippedStr) -> Self {
        self.description = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: PhoneNumberOptional) -> Self {
        self.phone = Some(value);
        self
    }

    pub fn sms_notifications_opted_in(mut self, value: bool) -> Self {
        self.sms_notifications_opted_in = Some(value);
        self
    }

    pub fn photo_url(mut self, value: impl Into<String>) -> Self {
        self.photo_url = Some(value.into());
        self
    }

    pub fn location_id(mut self, value: impl Into<String>) -> Self {
        self.location_id = Some(value.into());
        self
    }

    pub fn working_hours(mut self, value: WorkingHours) -> Self {
        self.working_hours = Some(value);
        self
    }

    pub fn display_on_booking_page(mut self, value: bool) -> Self {
        self.display_on_booking_page = Some(value);
        self
    }

    pub fn customer_facing_name(mut self, value: StrippedStr) -> Self {
        self.customer_facing_name = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: UnixTimestamp) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn updated_at_unix(mut self, value: UnixTimestamp) -> Self {
        self.updated_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionStaffResource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionStaffResourceBuilder::name)
    /// - [`id`](ReceptionStaffResourceBuilder::id)
    pub fn build(self) -> Result<ReceptionStaffResource, BuildError> {
        Ok(ReceptionStaffResource {
            groups: self.groups,
            ungrouped_rank: self.ungrouped_rank,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            is_active: self.is_active,
            email: self.email,
            phone: self.phone,
            sms_notifications_opted_in: self.sms_notifications_opted_in,
            photo_url: self.photo_url,
            location_id: self.location_id,
            working_hours: self.working_hours,
            display_on_booking_page: self.display_on_booking_page,
            customer_facing_name: self.customer_facing_name,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at_unix: self.created_at_unix,
            updated_at_unix: self.updated_at_unix,
        })
    }
}
