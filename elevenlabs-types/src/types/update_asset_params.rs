pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAssetParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateAssetParams {
    pub fn builder() -> UpdateAssetParamsBuilder {
        <UpdateAssetParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAssetParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateAssetParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAssetParams`].
    pub fn build(self) -> Result<UpdateAssetParams, BuildError> {
        Ok(UpdateAssetParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
