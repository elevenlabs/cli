pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum TemplateOutput {
        #[serde(rename = "array")]
        #[non_exhaustive]
        Array {
            #[serde(default)]
            id: String,
            status: TemplateRunStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<TemplateArrayOutputFailureReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<Vec<Box<TemplateOutput>>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            next_cursor: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            has_more: Option<bool>,
        },

        #[serde(rename = "audio")]
        #[non_exhaustive]
        Audio {
            #[serde(default)]
            id: String,
            status: TemplateRunStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<TemplateAudioOutputFailureReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_mime_type: Option<String>,
        },

        #[serde(rename = "boolean")]
        #[non_exhaustive]
        Boolean {
            #[serde(default)]
            id: String,
            status: TemplateRunStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<TemplateBooleanOutputFailureReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<bool>,
        },

        #[serde(rename = "image")]
        #[non_exhaustive]
        Image {
            #[serde(default)]
            id: String,
            status: TemplateRunStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<TemplateImageOutputFailureReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_mime_type: Option<String>,
        },

        #[serde(rename = "integer")]
        #[non_exhaustive]
        Integer {
            #[serde(default)]
            id: String,
            status: TemplateRunStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<TemplateIntegerOutputFailureReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<i64>,
        },

        #[serde(rename = "number")]
        #[non_exhaustive]
        Number {
            #[serde(default)]
            id: String,
            status: TemplateRunStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<TemplateNumberOutputFailureReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            content: Option<f64>,
        },

        #[serde(rename = "object")]
        #[non_exhaustive]
        Object {
            #[serde(default)]
            id: String,
            status: TemplateRunStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<TemplateObjectOutputFailureReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<HashMap<String, Option<Box<TemplateOutput>>>>,
        },

        #[serde(rename = "string")]
        #[non_exhaustive]
        r#String {
            #[serde(default)]
            id: String,
            status: TemplateRunStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<TemplateStringOutputFailureReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<String>,
        },

        #[serde(rename = "video")]
        #[non_exhaustive]
        Video {
            #[serde(default)]
            id: String,
            status: TemplateRunStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            failure_reason: Option<TemplateVideoOutputFailureReason>,
            #[serde(skip_serializing_if = "Option::is_none")]
            error_message: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            content_mime_type: Option<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl TemplateOutput {
    pub fn array(id: String, status: TemplateRunStatus) -> Self {
        Self::Array { id, status, failure_reason: None, error_message: None, content: None, next_cursor: None, has_more: None }
    }

    pub fn audio(id: String, status: TemplateRunStatus) -> Self {
        Self::Audio { id, status, failure_reason: None, error_message: None, content_url: None, content_mime_type: None }
    }

    pub fn boolean(id: String, status: TemplateRunStatus) -> Self {
        Self::Boolean { id, status, failure_reason: None, error_message: None, content: None }
    }

    pub fn image(id: String, status: TemplateRunStatus) -> Self {
        Self::Image { id, status, failure_reason: None, error_message: None, content_url: None, content_mime_type: None }
    }

    pub fn integer(id: String, status: TemplateRunStatus) -> Self {
        Self::Integer { id, status, failure_reason: None, error_message: None, content: None }
    }

    pub fn number(id: String, status: TemplateRunStatus) -> Self {
        Self::Number { id, status, failure_reason: None, error_message: None, content: None }
    }

    pub fn object(id: String, status: TemplateRunStatus) -> Self {
        Self::Object { id, status, failure_reason: None, error_message: None, content: None }
    }

    pub fn string(id: String, status: TemplateRunStatus) -> Self {
        Self::r#String { id, status, failure_reason: None, error_message: None, content: None }
    }

    pub fn video(id: String, status: TemplateRunStatus) -> Self {
        Self::Video { id, status, failure_reason: None, error_message: None, content_url: None, content_mime_type: None }
    }

    pub fn array_with_failure_reason(id: String, status: TemplateRunStatus, failure_reason: TemplateArrayOutputFailureReason, error_message: Option<String>, content: Option<Vec<Box<TemplateOutput>>>, next_cursor: Option<String>, has_more: Option<bool>) -> Self {
        Self::Array { id, status, failure_reason: Some(failure_reason), error_message, content, next_cursor, has_more }
    }

    pub fn array_with_error_message(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateArrayOutputFailureReason>, error_message: String, content: Option<Vec<Box<TemplateOutput>>>, next_cursor: Option<String>, has_more: Option<bool>) -> Self {
        Self::Array { id, status, failure_reason, error_message: Some(error_message), content, next_cursor, has_more }
    }

    pub fn array_with_content(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateArrayOutputFailureReason>, error_message: Option<String>, content: Vec<Box<TemplateOutput>>, next_cursor: Option<String>, has_more: Option<bool>) -> Self {
        Self::Array { id, status, failure_reason, error_message, content: Some(content), next_cursor, has_more }
    }

    pub fn array_with_next_cursor(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateArrayOutputFailureReason>, error_message: Option<String>, content: Option<Vec<Box<TemplateOutput>>>, next_cursor: String, has_more: Option<bool>) -> Self {
        Self::Array { id, status, failure_reason, error_message, content, next_cursor: Some(next_cursor), has_more }
    }

    pub fn array_with_has_more(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateArrayOutputFailureReason>, error_message: Option<String>, content: Option<Vec<Box<TemplateOutput>>>, next_cursor: Option<String>, has_more: bool) -> Self {
        Self::Array { id, status, failure_reason, error_message, content, next_cursor, has_more: Some(has_more) }
    }

    pub fn audio_with_failure_reason(id: String, status: TemplateRunStatus, failure_reason: TemplateAudioOutputFailureReason, error_message: Option<String>, content_url: Option<String>, content_mime_type: Option<String>) -> Self {
        Self::Audio { id, status, failure_reason: Some(failure_reason), error_message, content_url, content_mime_type }
    }

    pub fn audio_with_error_message(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateAudioOutputFailureReason>, error_message: String, content_url: Option<String>, content_mime_type: Option<String>) -> Self {
        Self::Audio { id, status, failure_reason, error_message: Some(error_message), content_url, content_mime_type }
    }

    pub fn audio_with_content_url(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateAudioOutputFailureReason>, error_message: Option<String>, content_url: String, content_mime_type: Option<String>) -> Self {
        Self::Audio { id, status, failure_reason, error_message, content_url: Some(content_url), content_mime_type }
    }

    pub fn audio_with_content_mime_type(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateAudioOutputFailureReason>, error_message: Option<String>, content_url: Option<String>, content_mime_type: String) -> Self {
        Self::Audio { id, status, failure_reason, error_message, content_url, content_mime_type: Some(content_mime_type) }
    }

    pub fn boolean_with_failure_reason(id: String, status: TemplateRunStatus, failure_reason: TemplateBooleanOutputFailureReason, error_message: Option<String>, content: Option<bool>) -> Self {
        Self::Boolean { id, status, failure_reason: Some(failure_reason), error_message, content }
    }

    pub fn boolean_with_error_message(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateBooleanOutputFailureReason>, error_message: String, content: Option<bool>) -> Self {
        Self::Boolean { id, status, failure_reason, error_message: Some(error_message), content }
    }

    pub fn boolean_with_content(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateBooleanOutputFailureReason>, error_message: Option<String>, content: bool) -> Self {
        Self::Boolean { id, status, failure_reason, error_message, content: Some(content) }
    }

    pub fn image_with_failure_reason(id: String, status: TemplateRunStatus, failure_reason: TemplateImageOutputFailureReason, error_message: Option<String>, content_url: Option<String>, content_mime_type: Option<String>) -> Self {
        Self::Image { id, status, failure_reason: Some(failure_reason), error_message, content_url, content_mime_type }
    }

    pub fn image_with_error_message(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateImageOutputFailureReason>, error_message: String, content_url: Option<String>, content_mime_type: Option<String>) -> Self {
        Self::Image { id, status, failure_reason, error_message: Some(error_message), content_url, content_mime_type }
    }

    pub fn image_with_content_url(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateImageOutputFailureReason>, error_message: Option<String>, content_url: String, content_mime_type: Option<String>) -> Self {
        Self::Image { id, status, failure_reason, error_message, content_url: Some(content_url), content_mime_type }
    }

    pub fn image_with_content_mime_type(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateImageOutputFailureReason>, error_message: Option<String>, content_url: Option<String>, content_mime_type: String) -> Self {
        Self::Image { id, status, failure_reason, error_message, content_url, content_mime_type: Some(content_mime_type) }
    }

    pub fn integer_with_failure_reason(id: String, status: TemplateRunStatus, failure_reason: TemplateIntegerOutputFailureReason, error_message: Option<String>, content: Option<i64>) -> Self {
        Self::Integer { id, status, failure_reason: Some(failure_reason), error_message, content }
    }

    pub fn integer_with_error_message(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateIntegerOutputFailureReason>, error_message: String, content: Option<i64>) -> Self {
        Self::Integer { id, status, failure_reason, error_message: Some(error_message), content }
    }

    pub fn integer_with_content(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateIntegerOutputFailureReason>, error_message: Option<String>, content: i64) -> Self {
        Self::Integer { id, status, failure_reason, error_message, content: Some(content) }
    }

    pub fn number_with_failure_reason(id: String, status: TemplateRunStatus, failure_reason: TemplateNumberOutputFailureReason, error_message: Option<String>, content: Option<f64>) -> Self {
        Self::Number { id, status, failure_reason: Some(failure_reason), error_message, content }
    }

    pub fn number_with_error_message(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateNumberOutputFailureReason>, error_message: String, content: Option<f64>) -> Self {
        Self::Number { id, status, failure_reason, error_message: Some(error_message), content }
    }

    pub fn number_with_content(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateNumberOutputFailureReason>, error_message: Option<String>, content: f64) -> Self {
        Self::Number { id, status, failure_reason, error_message, content: Some(content) }
    }

    pub fn object_with_failure_reason(id: String, status: TemplateRunStatus, failure_reason: TemplateObjectOutputFailureReason, error_message: Option<String>, content: Option<HashMap<String, Option<Box<TemplateOutput>>>>) -> Self {
        Self::Object { id, status, failure_reason: Some(failure_reason), error_message, content }
    }

    pub fn object_with_error_message(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateObjectOutputFailureReason>, error_message: String, content: Option<HashMap<String, Option<Box<TemplateOutput>>>>) -> Self {
        Self::Object { id, status, failure_reason, error_message: Some(error_message), content }
    }

    pub fn object_with_content(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateObjectOutputFailureReason>, error_message: Option<String>, content: HashMap<String, Option<Box<TemplateOutput>>>) -> Self {
        Self::Object { id, status, failure_reason, error_message, content: Some(content) }
    }

    pub fn string_with_failure_reason(id: String, status: TemplateRunStatus, failure_reason: TemplateStringOutputFailureReason, error_message: Option<String>, content: Option<String>) -> Self {
        Self::r#String { id, status, failure_reason: Some(failure_reason), error_message, content }
    }

    pub fn string_with_error_message(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateStringOutputFailureReason>, error_message: String, content: Option<String>) -> Self {
        Self::r#String { id, status, failure_reason, error_message: Some(error_message), content }
    }

    pub fn string_with_content(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateStringOutputFailureReason>, error_message: Option<String>, content: String) -> Self {
        Self::r#String { id, status, failure_reason, error_message, content: Some(content) }
    }

    pub fn video_with_failure_reason(id: String, status: TemplateRunStatus, failure_reason: TemplateVideoOutputFailureReason, error_message: Option<String>, content_url: Option<String>, content_mime_type: Option<String>) -> Self {
        Self::Video { id, status, failure_reason: Some(failure_reason), error_message, content_url, content_mime_type }
    }

    pub fn video_with_error_message(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateVideoOutputFailureReason>, error_message: String, content_url: Option<String>, content_mime_type: Option<String>) -> Self {
        Self::Video { id, status, failure_reason, error_message: Some(error_message), content_url, content_mime_type }
    }

    pub fn video_with_content_url(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateVideoOutputFailureReason>, error_message: Option<String>, content_url: String, content_mime_type: Option<String>) -> Self {
        Self::Video { id, status, failure_reason, error_message, content_url: Some(content_url), content_mime_type }
    }

    pub fn video_with_content_mime_type(id: String, status: TemplateRunStatus, failure_reason: Option<TemplateVideoOutputFailureReason>, error_message: Option<String>, content_url: Option<String>, content_mime_type: String) -> Self {
        Self::Video { id, status, failure_reason, error_message, content_url, content_mime_type: Some(content_mime_type) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
