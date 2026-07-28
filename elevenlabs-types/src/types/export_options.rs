pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "format")]
#[non_exhaustive]
pub enum ExportOptions {
        #[serde(rename = "docx")]
        #[non_exhaustive]
        Docx {
            #[serde(skip_serializing_if = "Option::is_none")]
            include_speakers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_timestamps: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            segment_on_silence_longer_than_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            max_segment_duration_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_segment_chars: Option<i64>,
        },

        #[serde(rename = "html")]
        #[non_exhaustive]
        Html {
            #[serde(skip_serializing_if = "Option::is_none")]
            include_speakers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_timestamps: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            segment_on_silence_longer_than_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            max_segment_duration_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_segment_chars: Option<i64>,
        },

        #[serde(rename = "pdf")]
        #[non_exhaustive]
        Pdf {
            #[serde(skip_serializing_if = "Option::is_none")]
            include_speakers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_timestamps: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            segment_on_silence_longer_than_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            max_segment_duration_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_segment_chars: Option<i64>,
        },

        #[serde(rename = "segmented_json")]
        #[non_exhaustive]
        SegmentedJson {
            #[serde(skip_serializing_if = "Option::is_none")]
            include_speakers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_timestamps: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            segment_on_silence_longer_than_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            max_segment_duration_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_segment_chars: Option<i64>,
        },

        #[serde(rename = "srt")]
        #[non_exhaustive]
        Srt {
            #[serde(skip_serializing_if = "Option::is_none")]
            max_characters_per_line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_speakers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_timestamps: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            segment_on_silence_longer_than_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            max_segment_duration_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_segment_chars: Option<i64>,
        },

        #[serde(rename = "txt")]
        #[non_exhaustive]
        Txt {
            #[serde(skip_serializing_if = "Option::is_none")]
            max_characters_per_line: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_speakers: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            include_timestamps: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            segment_on_silence_longer_than_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers::option")]
            max_segment_duration_s: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_segment_chars: Option<i64>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ExportOptions {
    pub fn docx() -> Self {
        Self::Docx { include_speakers: None, include_timestamps: None, segment_on_silence_longer_than_s: None, max_segment_duration_s: None, max_segment_chars: None }
    }

    pub fn html() -> Self {
        Self::Html { include_speakers: None, include_timestamps: None, segment_on_silence_longer_than_s: None, max_segment_duration_s: None, max_segment_chars: None }
    }

    pub fn pdf() -> Self {
        Self::Pdf { include_speakers: None, include_timestamps: None, segment_on_silence_longer_than_s: None, max_segment_duration_s: None, max_segment_chars: None }
    }

    pub fn segmented_json() -> Self {
        Self::SegmentedJson { include_speakers: None, include_timestamps: None, segment_on_silence_longer_than_s: None, max_segment_duration_s: None, max_segment_chars: None }
    }

    pub fn srt() -> Self {
        Self::Srt { max_characters_per_line: None, include_speakers: None, include_timestamps: None, segment_on_silence_longer_than_s: None, max_segment_duration_s: None, max_segment_chars: None }
    }

    pub fn txt() -> Self {
        Self::Txt { max_characters_per_line: None, include_speakers: None, include_timestamps: None, segment_on_silence_longer_than_s: None, max_segment_duration_s: None, max_segment_chars: None }
    }

    pub fn docx_with_include_speakers(include_speakers: bool, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Docx { include_speakers: Some(include_speakers), include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn docx_with_include_timestamps(include_speakers: Option<bool>, include_timestamps: bool, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Docx { include_speakers, include_timestamps: Some(include_timestamps), segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn docx_with_segment_on_silence_longer_than_s(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: f64, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Docx { include_speakers, include_timestamps, segment_on_silence_longer_than_s: Some(segment_on_silence_longer_than_s), max_segment_duration_s, max_segment_chars }
    }

    pub fn docx_with_max_segment_duration_s(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: f64, max_segment_chars: Option<i64>) -> Self {
        Self::Docx { include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s: Some(max_segment_duration_s), max_segment_chars }
    }

    pub fn docx_with_max_segment_chars(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: i64) -> Self {
        Self::Docx { include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars: Some(max_segment_chars) }
    }

    pub fn html_with_include_speakers(include_speakers: bool, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Html { include_speakers: Some(include_speakers), include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn html_with_include_timestamps(include_speakers: Option<bool>, include_timestamps: bool, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Html { include_speakers, include_timestamps: Some(include_timestamps), segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn html_with_segment_on_silence_longer_than_s(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: f64, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Html { include_speakers, include_timestamps, segment_on_silence_longer_than_s: Some(segment_on_silence_longer_than_s), max_segment_duration_s, max_segment_chars }
    }

    pub fn html_with_max_segment_duration_s(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: f64, max_segment_chars: Option<i64>) -> Self {
        Self::Html { include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s: Some(max_segment_duration_s), max_segment_chars }
    }

    pub fn html_with_max_segment_chars(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: i64) -> Self {
        Self::Html { include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars: Some(max_segment_chars) }
    }

    pub fn pdf_with_include_speakers(include_speakers: bool, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Pdf { include_speakers: Some(include_speakers), include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn pdf_with_include_timestamps(include_speakers: Option<bool>, include_timestamps: bool, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Pdf { include_speakers, include_timestamps: Some(include_timestamps), segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn pdf_with_segment_on_silence_longer_than_s(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: f64, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Pdf { include_speakers, include_timestamps, segment_on_silence_longer_than_s: Some(segment_on_silence_longer_than_s), max_segment_duration_s, max_segment_chars }
    }

    pub fn pdf_with_max_segment_duration_s(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: f64, max_segment_chars: Option<i64>) -> Self {
        Self::Pdf { include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s: Some(max_segment_duration_s), max_segment_chars }
    }

    pub fn pdf_with_max_segment_chars(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: i64) -> Self {
        Self::Pdf { include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars: Some(max_segment_chars) }
    }

    pub fn segmented_json_with_include_speakers(include_speakers: bool, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::SegmentedJson { include_speakers: Some(include_speakers), include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn segmented_json_with_include_timestamps(include_speakers: Option<bool>, include_timestamps: bool, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::SegmentedJson { include_speakers, include_timestamps: Some(include_timestamps), segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn segmented_json_with_segment_on_silence_longer_than_s(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: f64, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::SegmentedJson { include_speakers, include_timestamps, segment_on_silence_longer_than_s: Some(segment_on_silence_longer_than_s), max_segment_duration_s, max_segment_chars }
    }

    pub fn segmented_json_with_max_segment_duration_s(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: f64, max_segment_chars: Option<i64>) -> Self {
        Self::SegmentedJson { include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s: Some(max_segment_duration_s), max_segment_chars }
    }

    pub fn segmented_json_with_max_segment_chars(include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: i64) -> Self {
        Self::SegmentedJson { include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars: Some(max_segment_chars) }
    }

    pub fn srt_with_max_characters_per_line(max_characters_per_line: i64, include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Srt { max_characters_per_line: Some(max_characters_per_line), include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn srt_with_include_speakers(max_characters_per_line: Option<i64>, include_speakers: bool, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Srt { max_characters_per_line, include_speakers: Some(include_speakers), include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn srt_with_include_timestamps(max_characters_per_line: Option<i64>, include_speakers: Option<bool>, include_timestamps: bool, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Srt { max_characters_per_line, include_speakers, include_timestamps: Some(include_timestamps), segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn srt_with_segment_on_silence_longer_than_s(max_characters_per_line: Option<i64>, include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: f64, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Srt { max_characters_per_line, include_speakers, include_timestamps, segment_on_silence_longer_than_s: Some(segment_on_silence_longer_than_s), max_segment_duration_s, max_segment_chars }
    }

    pub fn srt_with_max_segment_duration_s(max_characters_per_line: Option<i64>, include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: f64, max_segment_chars: Option<i64>) -> Self {
        Self::Srt { max_characters_per_line, include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s: Some(max_segment_duration_s), max_segment_chars }
    }

    pub fn srt_with_max_segment_chars(max_characters_per_line: Option<i64>, include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: i64) -> Self {
        Self::Srt { max_characters_per_line, include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars: Some(max_segment_chars) }
    }

    pub fn txt_with_max_characters_per_line(max_characters_per_line: i64, include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Txt { max_characters_per_line: Some(max_characters_per_line), include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn txt_with_include_speakers(max_characters_per_line: Option<i64>, include_speakers: bool, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Txt { max_characters_per_line, include_speakers: Some(include_speakers), include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn txt_with_include_timestamps(max_characters_per_line: Option<i64>, include_speakers: Option<bool>, include_timestamps: bool, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Txt { max_characters_per_line, include_speakers, include_timestamps: Some(include_timestamps), segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars }
    }

    pub fn txt_with_segment_on_silence_longer_than_s(max_characters_per_line: Option<i64>, include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: f64, max_segment_duration_s: Option<f64>, max_segment_chars: Option<i64>) -> Self {
        Self::Txt { max_characters_per_line, include_speakers, include_timestamps, segment_on_silence_longer_than_s: Some(segment_on_silence_longer_than_s), max_segment_duration_s, max_segment_chars }
    }

    pub fn txt_with_max_segment_duration_s(max_characters_per_line: Option<i64>, include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: f64, max_segment_chars: Option<i64>) -> Self {
        Self::Txt { max_characters_per_line, include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s: Some(max_segment_duration_s), max_segment_chars }
    }

    pub fn txt_with_max_segment_chars(max_characters_per_line: Option<i64>, include_speakers: Option<bool>, include_timestamps: Option<bool>, segment_on_silence_longer_than_s: Option<f64>, max_segment_duration_s: Option<f64>, max_segment_chars: i64) -> Self {
        Self::Txt { max_characters_per_line, include_speakers, include_timestamps, segment_on_silence_longer_than_s, max_segment_duration_s, max_segment_chars: Some(max_segment_chars) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
