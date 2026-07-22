pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List scheduled group sessions for a group service in a date range.
/// 
/// Group services are scheduled in advance (e.g. yoga classes, workshops) and
/// callers register against an existing session. Use this for group services;
/// use ``check_service_availability`` for appointment / rental services.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListGroupSessionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListGroupSessionsParams {
    pub fn builder() -> ListGroupSessionsParamsBuilder {
        <ListGroupSessionsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListGroupSessionsParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ListGroupSessionsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListGroupSessionsParams`].
    pub fn build(self) -> Result<ListGroupSessionsParams, BuildError> {
        Ok(ListGroupSessionsParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
