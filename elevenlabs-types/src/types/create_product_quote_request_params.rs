pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProductQuoteRequestParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl CreateProductQuoteRequestParams {
    pub fn builder() -> CreateProductQuoteRequestParamsBuilder {
        <CreateProductQuoteRequestParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProductQuoteRequestParamsBuilder {
    smb_tool_type: Option<String>,
}

impl CreateProductQuoteRequestParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateProductQuoteRequestParams`].
    pub fn build(self) -> Result<CreateProductQuoteRequestParams, BuildError> {
        Ok(CreateProductQuoteRequestParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
