pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceApiKeyListResponseModel {
    #[serde(rename = "api-keys")]
    #[serde(default)]
    pub api_keys: Vec<WorkspaceApiKeyResponseModel>,
}

impl WorkspaceApiKeyListResponseModel {
    pub fn builder() -> WorkspaceApiKeyListResponseModelBuilder {
        <WorkspaceApiKeyListResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceApiKeyListResponseModelBuilder {
    api_keys: Option<Vec<WorkspaceApiKeyResponseModel>>,
}

impl WorkspaceApiKeyListResponseModelBuilder {
    pub fn api_keys(mut self, value: Vec<WorkspaceApiKeyResponseModel>) -> Self {
        self.api_keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceApiKeyListResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_keys`](WorkspaceApiKeyListResponseModelBuilder::api_keys)
    pub fn build(self) -> Result<WorkspaceApiKeyListResponseModel, BuildError> {
        Ok(WorkspaceApiKeyListResponseModel {
            api_keys: self.api_keys.ok_or_else(|| BuildError::missing_field("api_keys"))?,
        })
    }
}
