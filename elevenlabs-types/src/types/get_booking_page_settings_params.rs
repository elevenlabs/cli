pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBookingPageSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl GetBookingPageSettingsParams {
    pub fn builder() -> GetBookingPageSettingsParamsBuilder {
        <GetBookingPageSettingsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBookingPageSettingsParamsBuilder {
    smb_tool_type: Option<String>,
}

impl GetBookingPageSettingsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetBookingPageSettingsParams`].
    pub fn build(self) -> Result<GetBookingPageSettingsParams, BuildError> {
        Ok(GetBookingPageSettingsParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
