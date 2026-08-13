pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelCalendarEventParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CancelCalendarEventParams {
    pub fn builder() -> CancelCalendarEventParamsBuilder {
        <CancelCalendarEventParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelCalendarEventParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CancelCalendarEventParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelCalendarEventParams`].
    pub fn build(self) -> Result<CancelCalendarEventParams, BuildError> {
        Ok(CancelCalendarEventParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
