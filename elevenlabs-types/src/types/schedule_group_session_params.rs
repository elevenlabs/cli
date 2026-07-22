pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Schedule a single instance of a group service.
/// 
/// The session's duration is derived from the parent service so the assistant
/// only has to pin start time, the (optional) instructor / room, and the
/// location. Participants register separately via
/// ``register_for_group_session``.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleGroupSessionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ScheduleGroupSessionParams {
    pub fn builder() -> ScheduleGroupSessionParamsBuilder {
        <ScheduleGroupSessionParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleGroupSessionParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ScheduleGroupSessionParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleGroupSessionParams`].
    pub fn build(self) -> Result<ScheduleGroupSessionParams, BuildError> {
        Ok(ScheduleGroupSessionParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
