pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdPatch {
    /// Whether to archive the pronunciation dictionary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// The name of the pronunciation dictionary, used for identification only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdPatch {
    pub fn builder() -> BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdPatchBuilder {
        <BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdPatchBuilder {
    archived: Option<bool>,
    name: Option<String>,
}

impl BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdPatchBuilder {
    pub fn archived(mut self, value: bool) -> Self {
        self.archived = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdPatch`].
    pub fn build(self) -> Result<BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdPatch, BuildError> {
        Ok(BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdPatch {
            archived: self.archived,
            name: self.name,
        })
    }
}

