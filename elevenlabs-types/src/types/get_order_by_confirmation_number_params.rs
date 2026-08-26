pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrderByConfirmationNumberParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl GetOrderByConfirmationNumberParams {
    pub fn builder() -> GetOrderByConfirmationNumberParamsBuilder {
        <GetOrderByConfirmationNumberParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderByConfirmationNumberParamsBuilder {
    smb_tool_type: Option<String>,
}

impl GetOrderByConfirmationNumberParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderByConfirmationNumberParams`].
    pub fn build(self) -> Result<GetOrderByConfirmationNumberParams, BuildError> {
        Ok(GetOrderByConfirmationNumberParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
