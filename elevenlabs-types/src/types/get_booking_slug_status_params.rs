pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBookingSlugStatusParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl GetBookingSlugStatusParams {
    pub fn builder() -> GetBookingSlugStatusParamsBuilder {
        <GetBookingSlugStatusParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBookingSlugStatusParamsBuilder {
    smb_tool_type: Option<String>,
}

impl GetBookingSlugStatusParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetBookingSlugStatusParams`].
    pub fn build(self) -> Result<GetBookingSlugStatusParams, BuildError> {
        Ok(GetBookingSlugStatusParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
