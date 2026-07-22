pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceServiceAccountResponseModel {
    #[serde(default)]
    pub service_account_user_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<i64>,
    #[serde(rename = "api-keys")]
    #[serde(default)]
    pub api_keys: Vec<WorkspaceApiKeyResponseModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sharing_groups: Option<Vec<DefaultSharingGroupResponseModel>>,
}

impl WorkspaceServiceAccountResponseModel {
    pub fn builder() -> WorkspaceServiceAccountResponseModelBuilder {
        <WorkspaceServiceAccountResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceServiceAccountResponseModelBuilder {
    service_account_user_id: Option<String>,
    name: Option<String>,
    created_at_unix: Option<i64>,
    api_keys: Option<Vec<WorkspaceApiKeyResponseModel>>,
    default_sharing_groups: Option<Vec<DefaultSharingGroupResponseModel>>,
}

impl WorkspaceServiceAccountResponseModelBuilder {
    pub fn service_account_user_id(mut self, value: impl Into<String>) -> Self {
        self.service_account_user_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: i64) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn api_keys(mut self, value: Vec<WorkspaceApiKeyResponseModel>) -> Self {
        self.api_keys = Some(value);
        self
    }

    pub fn default_sharing_groups(mut self, value: Vec<DefaultSharingGroupResponseModel>) -> Self {
        self.default_sharing_groups = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceServiceAccountResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_account_user_id`](WorkspaceServiceAccountResponseModelBuilder::service_account_user_id)
    /// - [`name`](WorkspaceServiceAccountResponseModelBuilder::name)
    /// - [`api_keys`](WorkspaceServiceAccountResponseModelBuilder::api_keys)
    pub fn build(self) -> Result<WorkspaceServiceAccountResponseModel, BuildError> {
        Ok(WorkspaceServiceAccountResponseModel {
            service_account_user_id: self.service_account_user_id.ok_or_else(|| BuildError::missing_field("service_account_user_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            created_at_unix: self.created_at_unix,
            api_keys: self.api_keys.ok_or_else(|| BuildError::missing_field("api_keys"))?,
            default_sharing_groups: self.default_sharing_groups,
        })
    }
}
