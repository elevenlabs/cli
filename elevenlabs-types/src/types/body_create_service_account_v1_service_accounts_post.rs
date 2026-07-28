pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreateServiceAccountV1ServiceAccountsPost {
    #[serde(default)]
    pub name: String,
    /// List of groups with their permission levels to share with by default. Each entry should specify a group_id and a permission_level (admin, editor, or viewer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sharing_groups: Option<Vec<DefaultSharingGroupConfig>>,
}

impl BodyCreateServiceAccountV1ServiceAccountsPost {
    pub fn builder() -> BodyCreateServiceAccountV1ServiceAccountsPostBuilder {
        <BodyCreateServiceAccountV1ServiceAccountsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateServiceAccountV1ServiceAccountsPostBuilder {
    name: Option<String>,
    default_sharing_groups: Option<Vec<DefaultSharingGroupConfig>>,
}

impl BodyCreateServiceAccountV1ServiceAccountsPostBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn default_sharing_groups(mut self, value: Vec<DefaultSharingGroupConfig>) -> Self {
        self.default_sharing_groups = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateServiceAccountV1ServiceAccountsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BodyCreateServiceAccountV1ServiceAccountsPostBuilder::name)
    pub fn build(self) -> Result<BodyCreateServiceAccountV1ServiceAccountsPost, BuildError> {
        Ok(BodyCreateServiceAccountV1ServiceAccountsPost {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            default_sharing_groups: self.default_sharing_groups,
        })
    }
}

