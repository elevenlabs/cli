pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateLocationParams {
    pub fn builder() -> UpdateLocationParamsBuilder {
        <UpdateLocationParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLocationParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateLocationParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateLocationParams`].
    pub fn build(self) -> Result<UpdateLocationParams, BuildError> {
        Ok(UpdateLocationParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
