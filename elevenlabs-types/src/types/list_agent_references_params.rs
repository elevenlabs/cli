pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAgentReferencesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListAgentReferencesParams {
    pub fn builder() -> ListAgentReferencesParamsBuilder {
        <ListAgentReferencesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAgentReferencesParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ListAgentReferencesParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAgentReferencesParams`].
    pub fn build(self) -> Result<ListAgentReferencesParams, BuildError> {
        Ok(ListAgentReferencesParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
