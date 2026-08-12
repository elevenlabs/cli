pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetBookingSlugParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl SetBookingSlugParams {
    pub fn builder() -> SetBookingSlugParamsBuilder {
        <SetBookingSlugParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetBookingSlugParamsBuilder {
    smb_tool_type: Option<String>,
}

impl SetBookingSlugParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetBookingSlugParams`].
    pub fn build(self) -> Result<SetBookingSlugParams, BuildError> {
        Ok(SetBookingSlugParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
