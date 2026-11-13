pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReplaceWorkspaceConfigRequest {
    /// The `version` of the configuration being replaced. Omit it only to create the first configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_version: Option<i64>,
    #[serde(default)]
    pub config: CampaignConfigInput,
}

impl ReplaceWorkspaceConfigRequest {
    pub fn builder() -> ReplaceWorkspaceConfigRequestBuilder {
        <ReplaceWorkspaceConfigRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplaceWorkspaceConfigRequestBuilder {
    expected_version: Option<i64>,
    config: Option<CampaignConfigInput>,
}

impl ReplaceWorkspaceConfigRequestBuilder {
    pub fn expected_version(mut self, value: i64) -> Self {
        self.expected_version = Some(value);
        self
    }

    pub fn config(mut self, value: CampaignConfigInput) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplaceWorkspaceConfigRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](ReplaceWorkspaceConfigRequestBuilder::config)
    pub fn build(self) -> Result<ReplaceWorkspaceConfigRequest, BuildError> {
        Ok(ReplaceWorkspaceConfigRequest {
            expected_version: self.expected_version,
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}

