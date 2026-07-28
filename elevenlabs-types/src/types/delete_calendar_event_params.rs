pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Permanently remove a previously-cancelled calendar event.
/// 
/// This delete tool is the irreversible follow-up to cancel_calendar_event.
/// The backend rejects the call (422) if the event hasn't been
/// cancelled yet, so the only safe path is cancel-then-delete.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteCalendarEventParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl DeleteCalendarEventParams {
    pub fn builder() -> DeleteCalendarEventParamsBuilder {
        <DeleteCalendarEventParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteCalendarEventParamsBuilder {
    smb_tool_type: Option<String>,
}

impl DeleteCalendarEventParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteCalendarEventParams`].
    pub fn build(self) -> Result<DeleteCalendarEventParams, BuildError> {
        Ok(DeleteCalendarEventParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
