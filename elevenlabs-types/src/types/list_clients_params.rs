pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List clients ordered by most recently updated, with an optional limit.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListClientsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListClientsParams {
    pub fn builder() -> ListClientsParamsBuilder {
        <ListClientsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClientsParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ListClientsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListClientsParams`].
    pub fn build(self) -> Result<ListClientsParams, BuildError> {
        Ok(ListClientsParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
