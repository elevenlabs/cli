pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetScheduleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_location_filter: Option<bool>,
}

impl GetScheduleParams {
    pub fn builder() -> GetScheduleParamsBuilder {
        <GetScheduleParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetScheduleParamsBuilder {
    smb_tool_type: Option<String>,
    include_location_filter: Option<bool>,
}

impl GetScheduleParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    pub fn include_location_filter(mut self, value: bool) -> Self {
        self.include_location_filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetScheduleParams`].
    pub fn build(self) -> Result<GetScheduleParams, BuildError> {
        Ok(GetScheduleParams {
            smb_tool_type: self.smb_tool_type,
            include_location_filter: self.include_location_filter,
        })
    }
}
