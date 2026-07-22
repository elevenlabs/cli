pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPost {
    /// A list of pronunciation dictionary locators (pronunciation_dictionary_id, version_id) encoded as a list of JSON strings for pronunciation dictionaries to be applied to the text. A list of json encoded strings is required as adding projects may occur through formData as opposed to jsonBody. To specify multiple dictionaries use multiple --form lines in your curl, such as --form 'pronunciation_dictionary_locators="{\"pronunciation_dictionary_id\":\"Vmd4Zor6fplcA7WrINey\",\"version_id\":\"hRPaxjlTdR7wFMhV4w0b\"}"' --form 'pronunciation_dictionary_locators="{\"pronunciation_dictionary_id\":\"JzWtcGQMJ6bnlWwyMo7e\",\"version_id\":\"lbmwxiLu4q6txYxgdZqn\"}"'.
    #[serde(default)]
    pub pronunciation_dictionary_locators: Vec<PronunciationDictionaryVersionLocator>,
    /// This will automatically mark text in this project for reconversion when the new dictionary applies or the old one no longer does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidate_affected_text: Option<bool>,
}

impl BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPost {
    pub fn builder() -> BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPostBuilder {
        <BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPostBuilder {
    pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryVersionLocator>>,
    invalidate_affected_text: Option<bool>,
}

impl BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPostBuilder {
    pub fn pronunciation_dictionary_locators(mut self, value: Vec<PronunciationDictionaryVersionLocator>) -> Self {
        self.pronunciation_dictionary_locators = Some(value);
        self
    }

    pub fn invalidate_affected_text(mut self, value: bool) -> Self {
        self.invalidate_affected_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pronunciation_dictionary_locators`](BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPostBuilder::pronunciation_dictionary_locators)
    pub fn build(self) -> Result<BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPost, BuildError> {
        Ok(BodyCreatePronunciationDictionariesV1StudioProjectsProjectIdPronunciationDictionariesPost {
            pronunciation_dictionary_locators: self.pronunciation_dictionary_locators.ok_or_else(|| BuildError::missing_field("pronunciation_dictionary_locators"))?,
            invalidate_affected_text: self.invalidate_affected_text,
        })
    }
}

