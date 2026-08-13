pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The output of a prior generation on this API.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GenerationReference {
    /// The ID of the generation whose output to use, as returned when the generation was created.
    #[serde(default)]
    pub generation_id: String,
}

impl GenerationReference {
    pub fn builder() -> GenerationReferenceBuilder {
        <GenerationReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerationReferenceBuilder {
    generation_id: Option<String>,
}

impl GenerationReferenceBuilder {
    pub fn generation_id(mut self, value: impl Into<String>) -> Self {
        self.generation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GenerationReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generation_id`](GenerationReferenceBuilder::generation_id)
    pub fn build(self) -> Result<GenerationReference, BuildError> {
        Ok(GenerationReference {
            generation_id: self.generation_id.ok_or_else(|| BuildError::missing_field("generation_id"))?,
        })
    }
}
