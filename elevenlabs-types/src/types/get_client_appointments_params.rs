pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetClientAppointmentsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cancelled: Option<bool>,
}

impl GetClientAppointmentsParams {
    pub fn builder() -> GetClientAppointmentsParamsBuilder {
        <GetClientAppointmentsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetClientAppointmentsParamsBuilder {
    smb_tool_type: Option<String>,
    include_cancelled: Option<bool>,
}

impl GetClientAppointmentsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    pub fn include_cancelled(mut self, value: bool) -> Self {
        self.include_cancelled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetClientAppointmentsParams`].
    pub fn build(self) -> Result<GetClientAppointmentsParams, BuildError> {
        Ok(GetClientAppointmentsParams {
            smb_tool_type: self.smb_tool_type,
            include_cancelled: self.include_cancelled,
        })
    }
}
