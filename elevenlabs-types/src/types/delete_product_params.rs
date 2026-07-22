pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteProductParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl DeleteProductParams {
    pub fn builder() -> DeleteProductParamsBuilder {
        <DeleteProductParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteProductParamsBuilder {
    smb_tool_type: Option<String>,
}

impl DeleteProductParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteProductParams`].
    pub fn build(self) -> Result<DeleteProductParams, BuildError> {
        Ok(DeleteProductParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
