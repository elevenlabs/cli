pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Change the seat count of an existing group session registration.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateGroupSessionSeatsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateGroupSessionSeatsParams {
    pub fn builder() -> UpdateGroupSessionSeatsParamsBuilder {
        <UpdateGroupSessionSeatsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateGroupSessionSeatsParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateGroupSessionSeatsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateGroupSessionSeatsParams`].
    pub fn build(self) -> Result<UpdateGroupSessionSeatsParams, BuildError> {
        Ok(UpdateGroupSessionSeatsParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
