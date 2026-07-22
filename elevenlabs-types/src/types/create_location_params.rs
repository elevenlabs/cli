pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateLocationParams {
    pub fn builder() -> CreateLocationParamsBuilder {
        <CreateLocationParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateLocationParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateLocationParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateLocationParams`].
    pub fn build(self) -> Result<CreateLocationParams, BuildError> {
        Ok(CreateLocationParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
