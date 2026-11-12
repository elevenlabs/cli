pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceConfigResponse {
    /// Increases with every replacement; send it as `expected_version`.
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub config: CampaignConfigOutput,
}

impl WorkspaceConfigResponse {
    pub fn builder() -> WorkspaceConfigResponseBuilder {
        <WorkspaceConfigResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceConfigResponseBuilder {
    version: Option<i64>,
    config: Option<CampaignConfigOutput>,
}

impl WorkspaceConfigResponseBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn config(mut self, value: CampaignConfigOutput) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceConfigResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](WorkspaceConfigResponseBuilder::version)
    /// - [`config`](WorkspaceConfigResponseBuilder::config)
    pub fn build(self) -> Result<WorkspaceConfigResponse, BuildError> {
        Ok(WorkspaceConfigResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
