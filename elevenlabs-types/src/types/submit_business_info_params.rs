pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Finalize the onboarding interview: create a text knowledge source and mark done.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitBusinessInfoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl SubmitBusinessInfoParams {
    pub fn builder() -> SubmitBusinessInfoParamsBuilder {
        <SubmitBusinessInfoParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitBusinessInfoParamsBuilder {
    smb_tool_type: Option<String>,
}

impl SubmitBusinessInfoParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitBusinessInfoParams`].
    pub fn build(self) -> Result<SubmitBusinessInfoParams, BuildError> {
        Ok(SubmitBusinessInfoParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
