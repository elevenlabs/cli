pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Scrape-reachable subset of an asset (resource that can be booked).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceptionAssetScrapable {
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
}

impl ReceptionAssetScrapable {
    pub fn builder() -> ReceptionAssetScrapableBuilder {
        <ReceptionAssetScrapableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionAssetScrapableBuilder {
    groups: Option<Vec<ReceptionGroupMembership>>,
    ungrouped_rank: Option<i64>,
    name: Option<StrippedStr>,
    description: Option<StrippedStr>,
}

impl ReceptionAssetScrapableBuilder {
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

    /// Consumes the builder and constructs a [`ReceptionAssetScrapable`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionAssetScrapableBuilder::name)
    pub fn build(self) -> Result<ReceptionAssetScrapable, BuildError> {
        Ok(ReceptionAssetScrapable {
            groups: self.groups,
            ungrouped_rank: self.ungrouped_rank,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
