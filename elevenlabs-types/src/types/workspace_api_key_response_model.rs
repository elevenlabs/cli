pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceApiKeyResponseModel {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub hint: String,
    #[serde(default)]
    pub key_id: String,
    #[serde(default)]
    pub service_account_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<PermissionType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_reason: Option<LockReason>,
    /// Maximum number of credits allowed in the current billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_limit: Option<i64>,
    /// Credits already used in the current billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_count: Option<i64>,
    #[serde(default)]
    pub hashed_xi_api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_disable_allowed: Option<bool>,
}

impl WorkspaceApiKeyResponseModel {
    pub fn builder() -> WorkspaceApiKeyResponseModelBuilder {
        <WorkspaceApiKeyResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceApiKeyResponseModelBuilder {
    name: Option<String>,
    hint: Option<String>,
    key_id: Option<String>,
    service_account_user_id: Option<String>,
    created_at_unix: Option<i64>,
    is_disabled: Option<bool>,
    permissions: Option<Vec<PermissionType>>,
    disable_reason: Option<LockReason>,
    character_limit: Option<i64>,
    character_count: Option<i64>,
    hashed_xi_api_key: Option<String>,
    allowed_ips: Option<Vec<String>>,
    third_party_disable_allowed: Option<bool>,
}

impl WorkspaceApiKeyResponseModelBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn hint(mut self, value: impl Into<String>) -> Self {
        self.hint = Some(value.into());
        self
    }

    pub fn key_id(mut self, value: impl Into<String>) -> Self {
        self.key_id = Some(value.into());
        self
    }

    pub fn service_account_user_id(mut self, value: impl Into<String>) -> Self {
        self.service_account_user_id = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn is_disabled(mut self, value: bool) -> Self {
        self.is_disabled = Some(value);
        self
    }

    pub fn permissions(mut self, value: Vec<PermissionType>) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn disable_reason(mut self, value: LockReason) -> Self {
        self.disable_reason = Some(value);
        self
    }

    pub fn character_limit(mut self, value: i64) -> Self {
        self.character_limit = Some(value);
        self
    }

    pub fn character_count(mut self, value: i64) -> Self {
        self.character_count = Some(value);
        self
    }

    pub fn hashed_xi_api_key(mut self, value: impl Into<String>) -> Self {
        self.hashed_xi_api_key = Some(value.into());
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

    /// Consumes the builder and constructs a [`WorkspaceApiKeyResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](WorkspaceApiKeyResponseModelBuilder::name)
    /// - [`hint`](WorkspaceApiKeyResponseModelBuilder::hint)
    /// - [`key_id`](WorkspaceApiKeyResponseModelBuilder::key_id)
    /// - [`service_account_user_id`](WorkspaceApiKeyResponseModelBuilder::service_account_user_id)
    /// - [`hashed_xi_api_key`](WorkspaceApiKeyResponseModelBuilder::hashed_xi_api_key)
    pub fn build(self) -> Result<WorkspaceApiKeyResponseModel, BuildError> {
        Ok(WorkspaceApiKeyResponseModel {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            hint: self.hint.ok_or_else(|| BuildError::missing_field("hint"))?,
            key_id: self.key_id.ok_or_else(|| BuildError::missing_field("key_id"))?,
            service_account_user_id: self.service_account_user_id.ok_or_else(|| BuildError::missing_field("service_account_user_id"))?,
            created_at_unix: self.created_at_unix,
            is_disabled: self.is_disabled,
            permissions: self.permissions,
            disable_reason: self.disable_reason,
            character_limit: self.character_limit,
            character_count: self.character_count,
            hashed_xi_api_key: self.hashed_xi_api_key.ok_or_else(|| BuildError::missing_field("hashed_xi_api_key"))?,
            allowed_ips: self.allowed_ips,
            third_party_disable_allowed: self.third_party_disable_allowed,
        })
    }
}
