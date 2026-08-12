pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListHolidaysParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl ListHolidaysParams {
    pub fn builder() -> ListHolidaysParamsBuilder {
        <ListHolidaysParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListHolidaysParamsBuilder {
    smb_tool_type: Option<String>,
}

impl ListHolidaysParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListHolidaysParams`].
    pub fn build(self) -> Result<ListHolidaysParams, BuildError> {
        Ok(ListHolidaysParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
