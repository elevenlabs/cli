pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Create a new staff member in the system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateStaffParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
    /// Whether to offer a working_hours timezone, set from the workspace's enable_per_resource_timezones. While that is off every staff member uses the workspace timezone, so the key is not offered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_resource_timezones: Option<bool>,
}

impl CreateStaffParams {
    pub fn builder() -> CreateStaffParamsBuilder {
        <CreateStaffParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStaffParamsBuilder {
    smb_tool_type: Option<String>,
    per_resource_timezones: Option<bool>,
}

impl CreateStaffParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    pub fn per_resource_timezones(mut self, value: bool) -> Self {
        self.per_resource_timezones = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateStaffParams`].
    pub fn build(self) -> Result<CreateStaffParams, BuildError> {
        Ok(CreateStaffParams {
            smb_tool_type: self.smb_tool_type,
            per_resource_timezones: self.per_resource_timezones,
        })
    }
}
