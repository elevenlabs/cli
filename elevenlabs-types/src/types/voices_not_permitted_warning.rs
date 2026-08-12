pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VoicesNotPermittedWarning {
    /// Identifies this warning; branch on it to read the fields below.
    pub r#type: String,
    /// Speakers whose voices were not permitted for cloning. The dub used a replacement voice for each of them; the rest of the speakers are unaffected.
    #[serde(default)]
    pub speaker_ids: Vec<String>,
    /// Human-readable description of the warning, for display. The wording may change at any time; branch on `type` instead.
    #[serde(default)]
    pub message: String,
}

impl VoicesNotPermittedWarning {
    pub fn builder() -> VoicesNotPermittedWarningBuilder {
        <VoicesNotPermittedWarningBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoicesNotPermittedWarningBuilder {
    r#type: Option<String>,
    speaker_ids: Option<Vec<String>>,
    message: Option<String>,
}

impl VoicesNotPermittedWarningBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn speaker_ids(mut self, value: Vec<String>) -> Self {
        self.speaker_ids = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoicesNotPermittedWarning`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](VoicesNotPermittedWarningBuilder::r#type)
    /// - [`speaker_ids`](VoicesNotPermittedWarningBuilder::speaker_ids)
    /// - [`message`](VoicesNotPermittedWarningBuilder::message)
    pub fn build(self) -> Result<VoicesNotPermittedWarning, BuildError> {
        Ok(VoicesNotPermittedWarning {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            speaker_ids: self.speaker_ids.ok_or_else(|| BuildError::missing_field("speaker_ids"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
