pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetWorkspaceSecretsResponseModel {
    #[serde(default)]
    pub secrets: Vec<ConvAiWorkspaceStoredSecretConfig>,
    /// Cursor for fetching the next page of secrets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl GetWorkspaceSecretsResponseModel {
    pub fn builder() -> GetWorkspaceSecretsResponseModelBuilder {
        <GetWorkspaceSecretsResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetWorkspaceSecretsResponseModelBuilder {
    secrets: Option<Vec<ConvAiWorkspaceStoredSecretConfig>>,
    next_cursor: Option<String>,
}

impl GetWorkspaceSecretsResponseModelBuilder {
    pub fn secrets(mut self, value: Vec<ConvAiWorkspaceStoredSecretConfig>) -> Self {
        self.secrets = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetWorkspaceSecretsResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`secrets`](GetWorkspaceSecretsResponseModelBuilder::secrets)
    pub fn build(self) -> Result<GetWorkspaceSecretsResponseModel, BuildError> {
        Ok(GetWorkspaceSecretsResponseModel {
            secrets: self.secrets.ok_or_else(|| BuildError::missing_field("secrets"))?,
            next_cursor: self.next_cursor,
        })
    }
}
