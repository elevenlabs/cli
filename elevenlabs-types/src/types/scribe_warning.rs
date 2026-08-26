pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScribeWarning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(default)]
    pub warning: String,
}

impl ScribeWarning {
    pub fn builder() -> ScribeWarningBuilder {
        <ScribeWarningBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScribeWarningBuilder {
    message_type: Option<String>,
    warning: Option<String>,
}

impl ScribeWarningBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn warning(mut self, value: impl Into<String>) -> Self {
        self.warning = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScribeWarning`].
    /// This method will fail if any of the following fields are not set:
    /// - [`warning`](ScribeWarningBuilder::warning)
    pub fn build(self) -> Result<ScribeWarning, BuildError> {
        Ok(ScribeWarning {
            message_type: self.message_type,
            warning: self.warning.ok_or_else(|| BuildError::missing_field("warning"))?,
        })
    }
}
