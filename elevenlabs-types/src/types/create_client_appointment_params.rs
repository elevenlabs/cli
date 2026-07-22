pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateClientAppointmentParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateClientAppointmentParams {
    pub fn builder() -> CreateClientAppointmentParamsBuilder {
        <CreateClientAppointmentParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateClientAppointmentParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateClientAppointmentParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateClientAppointmentParams`].
    pub fn build(self) -> Result<CreateClientAppointmentParams, BuildError> {
        Ok(CreateClientAppointmentParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
