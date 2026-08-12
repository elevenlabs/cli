pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Search for clients by name, phone number, or email.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchClientsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl SearchClientsParams {
    pub fn builder() -> SearchClientsParamsBuilder {
        <SearchClientsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchClientsParamsBuilder {
    smb_tool_type: Option<String>,
}

impl SearchClientsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchClientsParams`].
    pub fn build(self) -> Result<SearchClientsParams, BuildError> {
        Ok(SearchClientsParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
