pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateHolidayParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateHolidayParams {
    pub fn builder() -> UpdateHolidayParamsBuilder {
        <UpdateHolidayParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateHolidayParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateHolidayParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateHolidayParams`].
    pub fn build(self) -> Result<UpdateHolidayParams, BuildError> {
        Ok(UpdateHolidayParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
