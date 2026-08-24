pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTransferRulesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListTransferRulesParams {
    pub fn builder() -> ListTransferRulesParamsBuilder {
        <ListTransferRulesParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTransferRulesParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ListTransferRulesParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListTransferRulesParams`].
    pub fn build(self) -> Result<ListTransferRulesParams, BuildError> {
        Ok(ListTransferRulesParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
