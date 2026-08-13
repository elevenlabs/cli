pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListClientInteractionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListClientInteractionsParams {
    pub fn builder() -> ListClientInteractionsParamsBuilder {
        <ListClientInteractionsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClientInteractionsParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ListClientInteractionsParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListClientInteractionsParams`].
    pub fn build(self) -> Result<ListClientInteractionsParams, BuildError> {
        Ok(ListClientInteractionsParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
