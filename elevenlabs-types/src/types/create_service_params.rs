pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Create a new service (classic, rental, or group) in the system.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateServiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateServiceParams {
    pub fn builder() -> CreateServiceParamsBuilder {
        <CreateServiceParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateServiceParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateServiceParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateServiceParams`].
    pub fn build(self) -> Result<CreateServiceParams, BuildError> {
        Ok(CreateServiceParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
