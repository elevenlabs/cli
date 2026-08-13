pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceCreateApiKeyResponseModel {
    #[serde(rename = "xi-api-key")]
    #[serde(default)]
    pub xi_api_key: String,
    #[serde(default)]
    pub key_id: String,
}

impl WorkspaceCreateApiKeyResponseModel {
    pub fn builder() -> WorkspaceCreateApiKeyResponseModelBuilder {
        <WorkspaceCreateApiKeyResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceCreateApiKeyResponseModelBuilder {
    xi_api_key: Option<String>,
    key_id: Option<String>,
}

impl WorkspaceCreateApiKeyResponseModelBuilder {
    pub fn xi_api_key(mut self, value: impl Into<String>) -> Self {
        self.xi_api_key = Some(value.into());
        self
    }

    pub fn key_id(mut self, value: impl Into<String>) -> Self {
        self.key_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceCreateApiKeyResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`xi_api_key`](WorkspaceCreateApiKeyResponseModelBuilder::xi_api_key)
    /// - [`key_id`](WorkspaceCreateApiKeyResponseModelBuilder::key_id)
    pub fn build(self) -> Result<WorkspaceCreateApiKeyResponseModel, BuildError> {
        Ok(WorkspaceCreateApiKeyResponseModel {
            xi_api_key: self.xi_api_key.ok_or_else(|| BuildError::missing_field("xi_api_key"))?,
            key_id: self.key_id.ok_or_else(|| BuildError::missing_field("key_id"))?,
        })
    }
}
