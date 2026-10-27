pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Every location, inactive ones included, with every owner-editable field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OwnerListLocationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl OwnerListLocationsParams {
    pub fn builder() -> OwnerListLocationsParamsBuilder {
        <OwnerListLocationsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnerListLocationsParamsBuilder {
    smb_tool_type: Option<String>,
}

impl OwnerListLocationsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OwnerListLocationsParams`].
    pub fn build(self) -> Result<OwnerListLocationsParams, BuildError> {
        Ok(OwnerListLocationsParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
