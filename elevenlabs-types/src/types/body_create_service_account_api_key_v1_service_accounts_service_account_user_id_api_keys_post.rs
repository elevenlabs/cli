pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPost {
    #[serde(default)]
    pub name: String,
    /// The permissions of the XI API.
    pub permissions: BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostPermissions,
    /// The character limit of the XI API key. If provided this will limit the usage of this api key to n characters per month where n is the chosen value. Requests that incur charges will fail after reaching this monthly limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_limit: Option<i64>,
    /// List of IP addresses or CIDR ranges allowed to use this API key. Each entry may be a CIDR range (e.g. '10.0.0.0/24') or a bare IP address (normalized to /32 or /128). On create, omit or pass null to allow all IPs. On update, omit to leave the allowlist unchanged, or pass "clear" to remove it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<Vec<String>>,
    /// Whether the holder of this key may disable it via the self-disable endpoint. On create, omit or pass null to use the workspace's default (enabled for non-Enterprise plans, disabled for Enterprise plans). On update, omit to leave it unchanged, or pass "clear" to reset it to the workspace default. Only honored for workspaces with self-disable access enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_disable_allowed: Option<bool>,
}

impl BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPost {
    pub fn builder() -> BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostBuilder {
        <BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostBuilder {
    name: Option<String>,
    permissions: Option<BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostPermissions>,
    character_limit: Option<i64>,
    allowed_ips: Option<Vec<String>>,
    third_party_disable_allowed: Option<bool>,
}

impl BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn permissions(mut self, value: BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostPermissions) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn character_limit(mut self, value: i64) -> Self {
        self.character_limit = Some(value);
        self
    }

    pub fn allowed_ips(mut self, value: Vec<String>) -> Self {
        self.allowed_ips = Some(value);
        self
    }

    pub fn third_party_disable_allowed(mut self, value: bool) -> Self {
        self.third_party_disable_allowed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostBuilder::name)
    /// - [`permissions`](BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPostBuilder::permissions)
    pub fn build(self) -> Result<BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPost, BuildError> {
        Ok(BodyCreateServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysPost {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            permissions: self.permissions.ok_or_else(|| BuildError::missing_field("permissions"))?,
            character_limit: self.character_limit,
            allowed_ips: self.allowed_ips,
            third_party_disable_allowed: self.third_party_disable_allowed,
        })
    }
}

