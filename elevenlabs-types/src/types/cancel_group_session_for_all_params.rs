pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Cancel an entire group session and notify every registered participant.
/// Destructive -- prefer ``cancel_group_session_registration`` for cancelling
/// a single attendee.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelGroupSessionForAllParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CancelGroupSessionForAllParams {
    pub fn builder() -> CancelGroupSessionForAllParamsBuilder {
        <CancelGroupSessionForAllParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelGroupSessionForAllParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CancelGroupSessionForAllParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelGroupSessionForAllParams`].
    pub fn build(self) -> Result<CancelGroupSessionForAllParams, BuildError> {
        Ok(CancelGroupSessionForAllParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
