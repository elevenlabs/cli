pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAssetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
    /// Whether to offer an available_hours timezone, set from the workspace's enable_per_resource_timezones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_resource_timezones: Option<bool>,
}

impl CreateAssetParams {
    pub fn builder() -> CreateAssetParamsBuilder {
        <CreateAssetParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAssetParamsBuilder {
    smb_tool_type: Option<String>,
    per_resource_timezones: Option<bool>,
}

impl CreateAssetParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    pub fn per_resource_timezones(mut self, value: bool) -> Self {
        self.per_resource_timezones = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAssetParams`].
    pub fn build(self) -> Result<CreateAssetParams, BuildError> {
        Ok(CreateAssetParams {
            smb_tool_type: self.smb_tool_type,
            per_resource_timezones: self.per_resource_timezones,
        })
    }
}
