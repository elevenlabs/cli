pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLocationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListLocationsParams {
    pub fn builder() -> ListLocationsParamsBuilder {
        <ListLocationsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLocationsParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ListLocationsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListLocationsParams`].
    pub fn build(self) -> Result<ListLocationsParams, BuildError> {
        Ok(ListLocationsParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
