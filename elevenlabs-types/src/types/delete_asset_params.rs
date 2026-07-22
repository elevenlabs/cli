pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAssetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl DeleteAssetParams {
    pub fn builder() -> DeleteAssetParamsBuilder {
        <DeleteAssetParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAssetParamsBuilder {
    smb_tool_type: Option<String>,
}

impl DeleteAssetParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAssetParams`].
    pub fn build(self) -> Result<DeleteAssetParams, BuildError> {
        Ok(DeleteAssetParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
