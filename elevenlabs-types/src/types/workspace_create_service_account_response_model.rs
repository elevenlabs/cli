pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceCreateServiceAccountResponseModel {
    #[serde(rename = "service-account-user-id")]
    #[serde(default)]
    pub service_account_user_id: String,
}

impl WorkspaceCreateServiceAccountResponseModel {
    pub fn builder() -> WorkspaceCreateServiceAccountResponseModelBuilder {
        <WorkspaceCreateServiceAccountResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceCreateServiceAccountResponseModelBuilder {
    service_account_user_id: Option<String>,
}

impl WorkspaceCreateServiceAccountResponseModelBuilder {
    pub fn service_account_user_id(mut self, value: impl Into<String>) -> Self {
        self.service_account_user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceCreateServiceAccountResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_account_user_id`](WorkspaceCreateServiceAccountResponseModelBuilder::service_account_user_id)
    pub fn build(self) -> Result<WorkspaceCreateServiceAccountResponseModel, BuildError> {
        Ok(WorkspaceCreateServiceAccountResponseModel {
            service_account_user_id: self.service_account_user_id.ok_or_else(|| BuildError::missing_field("service_account_user_id"))?,
        })
    }
}
