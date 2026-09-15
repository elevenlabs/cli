pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendCustomEmailParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl SendCustomEmailParams {
    pub fn builder() -> SendCustomEmailParamsBuilder {
        <SendCustomEmailParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendCustomEmailParamsBuilder {
    smb_tool_type: Option<String>,
}

impl SendCustomEmailParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendCustomEmailParams`].
    pub fn build(self) -> Result<SendCustomEmailParams, BuildError> {
        Ok(SendCustomEmailParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
