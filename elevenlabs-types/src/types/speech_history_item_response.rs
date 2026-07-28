pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpeechHistoryItemResponse {
    /// The ID of the history item.
    #[serde(default)]
    pub history_item_id: String,
    /// The ID of the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The ID of the voice used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// The ID of the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// The name of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_name: Option<String>,
    /// The category of the voice. Either 'premade', 'cloned', 'generated' or 'professional'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_category: Option<SpeechHistoryItemResponseVoiceCategory>,
    /// The text used to generate the audio item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Unix timestamp of when the item was created.
    #[serde(default)]
    pub date_unix: i64,
    /// The character count change from.
    #[serde(default)]
    pub character_count_change_from: i64,
    /// The character count change to.
    #[serde(default)]
    pub character_count_change_to: i64,
    /// The content type of the generated item.
    #[serde(default)]
    pub content_type: String,
    pub state: serde_json::Value,
    /// The settings of the history item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<HashMap<String, serde_json::Value>>,
    /// The ID of the share link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_link_id: Option<String>,
    /// The source of the history item. Either TTS (text to speech), STS (speech to text), AN (audio native), Projects, Dubbing, PlayAPI, PD (pronunciation dictionary) or ConvAI (Agents Platform).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SpeechHistoryItemResponseSource>,
    /// The alignments of the history item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignments: Option<HistoryAlignmentsResponseModel>,
    /// The dialogue (voice and text pairs) used to generate the audio item. If this is set then the top level `text` and `voice_id` fields will be empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialogue: Option<Vec<DialogueInputResponseModel>>,
    /// The output format the audio was originally generated in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
}

impl SpeechHistoryItemResponse {
    pub fn builder() -> SpeechHistoryItemResponseBuilder {
        <SpeechHistoryItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpeechHistoryItemResponseBuilder {
    history_item_id: Option<String>,
    request_id: Option<String>,
    voice_id: Option<String>,
    model_id: Option<String>,
    voice_name: Option<String>,
    voice_category: Option<SpeechHistoryItemResponseVoiceCategory>,
    text: Option<String>,
    date_unix: Option<i64>,
    character_count_change_from: Option<i64>,
    character_count_change_to: Option<i64>,
    content_type: Option<String>,
    state: Option<serde_json::Value>,
    settings: Option<HashMap<String, serde_json::Value>>,
    share_link_id: Option<String>,
    source: Option<SpeechHistoryItemResponseSource>,
    alignments: Option<HistoryAlignmentsResponseModel>,
    dialogue: Option<Vec<DialogueInputResponseModel>>,
    output_format: Option<String>,
}

impl SpeechHistoryItemResponseBuilder {
    pub fn history_item_id(mut self, value: impl Into<String>) -> Self {
        self.history_item_id = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn voice_name(mut self, value: impl Into<String>) -> Self {
        self.voice_name = Some(value.into());
        self
    }

    pub fn voice_category(mut self, value: SpeechHistoryItemResponseVoiceCategory) -> Self {
        self.voice_category = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn date_unix(mut self, value: i64) -> Self {
        self.date_unix = Some(value);
        self
    }

    pub fn character_count_change_from(mut self, value: i64) -> Self {
        self.character_count_change_from = Some(value);
        self
    }

    pub fn character_count_change_to(mut self, value: i64) -> Self {
        self.character_count_change_to = Some(value);
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn state(mut self, value: serde_json::Value) -> Self {
        self.state = Some(value);
        self
    }

    pub fn settings(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn share_link_id(mut self, value: impl Into<String>) -> Self {
        self.share_link_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: SpeechHistoryItemResponseSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn alignments(mut self, value: HistoryAlignmentsResponseModel) -> Self {
        self.alignments = Some(value);
        self
    }

    pub fn dialogue(mut self, value: Vec<DialogueInputResponseModel>) -> Self {
        self.dialogue = Some(value);
        self
    }

    pub fn output_format(mut self, value: impl Into<String>) -> Self {
        self.output_format = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpeechHistoryItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`history_item_id`](SpeechHistoryItemResponseBuilder::history_item_id)
    /// - [`date_unix`](SpeechHistoryItemResponseBuilder::date_unix)
    /// - [`character_count_change_from`](SpeechHistoryItemResponseBuilder::character_count_change_from)
    /// - [`character_count_change_to`](SpeechHistoryItemResponseBuilder::character_count_change_to)
    /// - [`content_type`](SpeechHistoryItemResponseBuilder::content_type)
    /// - [`state`](SpeechHistoryItemResponseBuilder::state)
    pub fn build(self) -> Result<SpeechHistoryItemResponse, BuildError> {
        Ok(SpeechHistoryItemResponse {
            history_item_id: self.history_item_id.ok_or_else(|| BuildError::missing_field("history_item_id"))?,
            request_id: self.request_id,
            voice_id: self.voice_id,
            model_id: self.model_id,
            voice_name: self.voice_name,
            voice_category: self.voice_category,
            text: self.text,
            date_unix: self.date_unix.ok_or_else(|| BuildError::missing_field("date_unix"))?,
            character_count_change_from: self.character_count_change_from.ok_or_else(|| BuildError::missing_field("character_count_change_from"))?,
            character_count_change_to: self.character_count_change_to.ok_or_else(|| BuildError::missing_field("character_count_change_to"))?,
            content_type: self.content_type.ok_or_else(|| BuildError::missing_field("content_type"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            settings: self.settings,
            share_link_id: self.share_link_id,
            source: self.source,
            alignments: self.alignments,
            dialogue: self.dialogue,
            output_format: self.output_format,
        })
    }
}
