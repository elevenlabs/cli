pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckServiceAvailabilityParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CheckServiceAvailabilityParams {
    pub fn builder() -> CheckServiceAvailabilityParamsBuilder {
        <CheckServiceAvailabilityParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckServiceAvailabilityParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CheckServiceAvailabilityParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckServiceAvailabilityParams`].
    pub fn build(self) -> Result<CheckServiceAvailabilityParams, BuildError> {
        Ok(CheckServiceAvailabilityParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
