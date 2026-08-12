pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TriggerUserVerificationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl TriggerUserVerificationParams {
    pub fn builder() -> TriggerUserVerificationParamsBuilder {
        <TriggerUserVerificationParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TriggerUserVerificationParamsBuilder {
    smb_tool_type: Option<String>,
}

impl TriggerUserVerificationParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TriggerUserVerificationParams`].
    pub fn build(self) -> Result<TriggerUserVerificationParams, BuildError> {
        Ok(TriggerUserVerificationParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
