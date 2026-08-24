pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateProductParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateProductParams {
    pub fn builder() -> UpdateProductParamsBuilder {
        <UpdateProductParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateProductParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateProductParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateProductParams`].
    pub fn build(self) -> Result<UpdateProductParams, BuildError> {
        Ok(UpdateProductParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
