pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCalendarEventParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateCalendarEventParams {
    pub fn builder() -> UpdateCalendarEventParamsBuilder {
        <UpdateCalendarEventParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCalendarEventParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateCalendarEventParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCalendarEventParams`].
    pub fn build(self) -> Result<UpdateCalendarEventParams, BuildError> {
        Ok(UpdateCalendarEventParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
