pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateServiceQuoteRequestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateServiceQuoteRequestParams {
    pub fn builder() -> CreateServiceQuoteRequestParamsBuilder {
        <CreateServiceQuoteRequestParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateServiceQuoteRequestParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateServiceQuoteRequestParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateServiceQuoteRequestParams`].
    pub fn build(self) -> Result<CreateServiceQuoteRequestParams, BuildError> {
        Ok(CreateServiceQuoteRequestParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
