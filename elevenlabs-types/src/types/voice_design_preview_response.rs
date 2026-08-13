pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceDesignPreviewResponse {
    /// The previews of the generated voices.
    #[serde(default)]
    pub previews: Vec<VoicePreviewResponseModel>,
    /// The text used to preview the voices.
    #[serde(default)]
    pub text: String,
}

impl VoiceDesignPreviewResponse {
    pub fn builder() -> VoiceDesignPreviewResponseBuilder {
        <VoiceDesignPreviewResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceDesignPreviewResponseBuilder {
    previews: Option<Vec<VoicePreviewResponseModel>>,
    text: Option<String>,
}

impl VoiceDesignPreviewResponseBuilder {
    pub fn previews(mut self, value: Vec<VoicePreviewResponseModel>) -> Self {
        self.previews = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoiceDesignPreviewResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`previews`](VoiceDesignPreviewResponseBuilder::previews)
    /// - [`text`](VoiceDesignPreviewResponseBuilder::text)
    pub fn build(self) -> Result<VoiceDesignPreviewResponse, BuildError> {
        Ok(VoiceDesignPreviewResponse {
            previews: self.previews.ok_or_else(|| BuildError::missing_field("previews"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
