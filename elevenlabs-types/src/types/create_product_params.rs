pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProductParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateProductParams {
    pub fn builder() -> CreateProductParamsBuilder {
        <CreateProductParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProductParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateProductParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateProductParams`].
    pub fn build(self) -> Result<CreateProductParams, BuildError> {
        Ok(CreateProductParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
