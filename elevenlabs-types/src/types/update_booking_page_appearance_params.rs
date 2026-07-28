pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBookingPageAppearanceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_tool_type: Option<String>,
}

impl UpdateBookingPageAppearanceParams {
    pub fn builder() -> UpdateBookingPageAppearanceParamsBuilder {
        <UpdateBookingPageAppearanceParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBookingPageAppearanceParamsBuilder {
    smb_tool_type: Option<String>,
}

impl UpdateBookingPageAppearanceParamsBuilder {
    pub fn smb_tool_type(mut self, value: impl Into<String>) -> Self {
        self.smb_tool_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateBookingPageAppearanceParams`].
    pub fn build(self) -> Result<UpdateBookingPageAppearanceParams, BuildError> {
        Ok(UpdateBookingPageAppearanceParams {
            smb_tool_type: self.smb_tool_type,
        })
    }
}
