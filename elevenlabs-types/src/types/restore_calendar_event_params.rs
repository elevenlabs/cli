pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RestoreCalendarEventParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl RestoreCalendarEventParams {
    pub fn builder() -> RestoreCalendarEventParamsBuilder {
        <RestoreCalendarEventParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RestoreCalendarEventParamsBuilder {
    smb_tool_type: Option<String>,
}

impl RestoreCalendarEventParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RestoreCalendarEventParams`].
    pub fn build(self) -> Result<RestoreCalendarEventParams, BuildError> {
        Ok(RestoreCalendarEventParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
