pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OTelStatus {
    #[serde(default)]
    pub code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl OTelStatus {
    pub fn builder() -> OTelStatusBuilder {
        <OTelStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OTelStatusBuilder {
    code: Option<i64>,
    message: Option<String>,
}

impl OTelStatusBuilder {
    pub fn code(mut self, value: i64) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OTelStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](OTelStatusBuilder::code)
    pub fn build(self) -> Result<OTelStatus, BuildError> {
        Ok(OTelStatus {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message,
        })
    }
}
