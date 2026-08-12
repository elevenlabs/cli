pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Update an existing service's information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateServiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateServiceParams {
    pub fn builder() -> UpdateServiceParamsBuilder {
        <UpdateServiceParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateServiceParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateServiceParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateServiceParams`].
    pub fn build(self) -> Result<UpdateServiceParams, BuildError> {
        Ok(UpdateServiceParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
