pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBookingPageSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateBookingPageSettingsParams {
    pub fn builder() -> UpdateBookingPageSettingsParamsBuilder {
        <UpdateBookingPageSettingsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBookingPageSettingsParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateBookingPageSettingsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateBookingPageSettingsParams`].
    pub fn build(self) -> Result<UpdateBookingPageSettingsParams, BuildError> {
        Ok(UpdateBookingPageSettingsParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
