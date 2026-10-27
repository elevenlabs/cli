pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetClientAppointmentsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cancelled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl GetClientAppointmentsParams {
    pub fn builder() -> GetClientAppointmentsParamsBuilder {
        <GetClientAppointmentsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetClientAppointmentsParamsBuilder {
    include_cancelled: Option<bool>,
    smb_tool_type: Option<String>,
}

impl GetClientAppointmentsParamsBuilder {
    pub fn include_cancelled(mut self, value: bool) -> Self {
        self.include_cancelled = Some(value);
        self
    }

    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetClientAppointmentsParams`].
    pub fn build(self) -> Result<GetClientAppointmentsParams, BuildError> {
        Ok(GetClientAppointmentsParams {
            include_cancelled: self.include_cancelled,
            smb_tool_type: self.smb_tool_type,
        })
    }
}
