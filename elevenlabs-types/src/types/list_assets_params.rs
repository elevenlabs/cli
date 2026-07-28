pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAssetsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_kwargs: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListAssetsParams {
    pub fn builder() -> ListAssetsParamsBuilder {
        <ListAssetsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAssetsParamsBuilder {
    list_kwargs: Option<HashMap<String, serde_json::Value>>,
    smb_tool_type: Option<String>,
}

impl ListAssetsParamsBuilder {
    pub fn list_kwargs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.list_kwargs = Some(value);
        self
    }

    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListAssetsParams`].
    pub fn build(self) -> Result<ListAssetsParams, BuildError> {
        Ok(ListAssetsParams {
            list_kwargs: self.list_kwargs,
            smb_tool_type: self.smb_tool_type,
        })
    }
}
