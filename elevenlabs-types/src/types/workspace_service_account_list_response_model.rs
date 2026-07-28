pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceServiceAccountListResponseModel {
    #[serde(rename = "service-accounts")]
    #[serde(default)]
    pub service_accounts: Vec<WorkspaceServiceAccountResponseModel>,
}

impl WorkspaceServiceAccountListResponseModel {
    pub fn builder() -> WorkspaceServiceAccountListResponseModelBuilder {
        <WorkspaceServiceAccountListResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceServiceAccountListResponseModelBuilder {
    service_accounts: Option<Vec<WorkspaceServiceAccountResponseModel>>,
}

impl WorkspaceServiceAccountListResponseModelBuilder {
    pub fn service_accounts(mut self, value: Vec<WorkspaceServiceAccountResponseModel>) -> Self {
        self.service_accounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceServiceAccountListResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_accounts`](WorkspaceServiceAccountListResponseModelBuilder::service_accounts)
    pub fn build(self) -> Result<WorkspaceServiceAccountListResponseModel, BuildError> {
        Ok(WorkspaceServiceAccountListResponseModel {
            service_accounts: self.service_accounts.ok_or_else(|| BuildError::missing_field("service_accounts"))?,
        })
    }
}
