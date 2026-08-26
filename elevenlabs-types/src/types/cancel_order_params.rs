pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelOrderParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CancelOrderParams {
    pub fn builder() -> CancelOrderParamsBuilder {
        <CancelOrderParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CancelOrderParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderParams`].
    pub fn build(self) -> Result<CancelOrderParams, BuildError> {
        Ok(CancelOrderParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
