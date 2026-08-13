pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteHolidayParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl DeleteHolidayParams {
    pub fn builder() -> DeleteHolidayParamsBuilder {
        <DeleteHolidayParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteHolidayParamsBuilder {
    smb_tool_type: Option<String>,
}

impl DeleteHolidayParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteHolidayParams`].
    pub fn build(self) -> Result<DeleteHolidayParams, BuildError> {
        Ok(DeleteHolidayParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
