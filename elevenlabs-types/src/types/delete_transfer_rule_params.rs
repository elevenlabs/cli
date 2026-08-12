pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteTransferRuleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl DeleteTransferRuleParams {
    pub fn builder() -> DeleteTransferRuleParamsBuilder {
        <DeleteTransferRuleParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteTransferRuleParamsBuilder {
    smb_tool_type: Option<String>,
}

impl DeleteTransferRuleParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteTransferRuleParams`].
    pub fn build(self) -> Result<DeleteTransferRuleParams, BuildError> {
        Ok(DeleteTransferRuleParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
