pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DuplicateSpeechEngineRequest {
    /// Name for the duplicated speech engine. Defaults to the source name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DuplicateSpeechEngineRequest {
    pub fn builder() -> DuplicateSpeechEngineRequestBuilder {
        <DuplicateSpeechEngineRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateSpeechEngineRequestBuilder {
    name: Option<String>,
}

impl DuplicateSpeechEngineRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DuplicateSpeechEngineRequest`].
    pub fn build(self) -> Result<DuplicateSpeechEngineRequest, BuildError> {
        Ok(DuplicateSpeechEngineRequest {
            name: self.name,
        })
    }
}
