pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Represents a physical resource that can be booked (rooms, equipment, vehicles).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceptionAssetResource {
    /// Group memberships for this item. Each entry carries the group label and the item's 0-based rank within that group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ReceptionGroupMembership>>,
    /// Display position among items with no group; `null` sorts last. Change it with the reorder endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ungrouped_rank: Option<i64>,
    /// Internal/admin name of the asset, shown in the dashboard and owner notifications.
    #[serde(default)]
    pub name: StrippedStr,
    /// Description of the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<StrippedStr>,
    /// Whether the asset can be booked. Inactive assets keep their booking history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// Custom availability hours for this asset. Null means follow the assigned location's operating_hours (requires location_id).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_hours: Option<WorkingHours>,
    /// URL of a photo of the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Location this asset belongs to; empty means available at all locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    /// Optional public-facing label. When empty, `name` is shown to customers too. `name` stays the internal/admin label used in the dashboard and business-owner notifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_facing_name: Option<StrippedStr>,
    /// The asset's ID.
    #[serde(default)]
    pub id: String,
    /// When this was created, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<UnixTimestamp>,
    /// When this was last updated, as a Unix timestamp in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_unix: Option<UnixTimestamp>,
}

impl ReceptionAssetResource {
    pub fn builder() -> ReceptionAssetResourceBuilder {
        <ReceptionAssetResourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionAssetResourceBuilder {
    groups: Option<Vec<ReceptionGroupMembership>>,
    ungrouped_rank: Option<i64>,
    name: Option<StrippedStr>,
    description: Option<StrippedStr>,
    is_active: Option<bool>,
    available_hours: Option<WorkingHours>,
    photo_url: Option<String>,
    location_id: Option<String>,
    customer_facing_name: Option<StrippedStr>,
    id: Option<String>,
    created_at_unix: Option<UnixTimestamp>,
    updated_at_unix: Option<UnixTimestamp>,
}

impl ReceptionAssetResourceBuilder {
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

    pub fn available_hours(mut self, value: WorkingHours) -> Self {
        self.available_hours = Some(value);
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

    /// Consumes the builder and constructs a [`ReceptionAssetResource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionAssetResourceBuilder::name)
    /// - [`id`](ReceptionAssetResourceBuilder::id)
    pub fn build(self) -> Result<ReceptionAssetResource, BuildError> {
        Ok(ReceptionAssetResource {
            groups: self.groups,
            ungrouped_rank: self.ungrouped_rank,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            is_active: self.is_active,
            available_hours: self.available_hours,
            photo_url: self.photo_url,
            location_id: self.location_id,
            customer_facing_name: self.customer_facing_name,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at_unix: self.created_at_unix,
            updated_at_unix: self.updated_at_unix,
        })
    }
}
