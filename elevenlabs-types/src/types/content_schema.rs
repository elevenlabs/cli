pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ContentSchema {
        #[serde(rename = "array")]
        #[non_exhaustive]
        Array {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            items: Box<ContentSchema>,
        },

        #[serde(rename = "audio")]
        #[non_exhaustive]
        Audio {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
        },

        #[serde(rename = "boolean")]
        #[non_exhaustive]
        Boolean {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
        },

        #[serde(rename = "image")]
        #[non_exhaustive]
        Image {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
        },

        #[serde(rename = "integer")]
        #[non_exhaustive]
        Integer {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
        },

        #[serde(rename = "number")]
        #[non_exhaustive]
        Number {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            r#enum: Option<Vec<f64>>,
        },

        #[serde(rename = "object")]
        #[non_exhaustive]
        Object {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            properties: Option<HashMap<String, Box<ContentSchema>>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            required: Option<Vec<String>>,
        },

        #[serde(rename = "string")]
        #[non_exhaustive]
        r#String {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            r#enum: Option<Vec<String>>,
        },

        #[serde(rename = "video")]
        #[non_exhaustive]
        Video {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
        },

        #[serde(rename = "voice")]
        #[non_exhaustive]
        Voice {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ContentSchema {
    pub fn array(items: Box<ContentSchema>) -> Self {
        Self::Array { title: None, description: None, items }
    }

    pub fn audio() -> Self {
        Self::Audio { title: None, description: None }
    }

    pub fn boolean() -> Self {
        Self::Boolean { title: None, description: None }
    }

    pub fn image() -> Self {
        Self::Image { title: None, description: None }
    }

    pub fn integer() -> Self {
        Self::Integer { title: None, description: None }
    }

    pub fn number() -> Self {
        Self::Number { title: None, description: None, r#enum: None }
    }

    pub fn object() -> Self {
        Self::Object { title: None, description: None, properties: None, required: None }
    }

    pub fn string() -> Self {
        Self::r#String { title: None, description: None, r#enum: None }
    }

    pub fn video() -> Self {
        Self::Video { title: None, description: None }
    }

    pub fn voice() -> Self {
        Self::Voice { title: None, description: None }
    }

    pub fn array_with_title(title: String, description: Option<String>, items: Box<ContentSchema>) -> Self {
        Self::Array { title: Some(title), description, items }
    }

    pub fn array_with_description(title: Option<String>, description: String, items: Box<ContentSchema>) -> Self {
        Self::Array { title, description: Some(description), items }
    }

    pub fn audio_with_title(title: String, description: Option<String>) -> Self {
        Self::Audio { title: Some(title), description }
    }

    pub fn audio_with_description(title: Option<String>, description: String) -> Self {
        Self::Audio { title, description: Some(description) }
    }

    pub fn boolean_with_title(title: String, description: Option<String>) -> Self {
        Self::Boolean { title: Some(title), description }
    }

    pub fn boolean_with_description(title: Option<String>, description: String) -> Self {
        Self::Boolean { title, description: Some(description) }
    }

    pub fn image_with_title(title: String, description: Option<String>) -> Self {
        Self::Image { title: Some(title), description }
    }

    pub fn image_with_description(title: Option<String>, description: String) -> Self {
        Self::Image { title, description: Some(description) }
    }

    pub fn integer_with_title(title: String, description: Option<String>) -> Self {
        Self::Integer { title: Some(title), description }
    }

    pub fn integer_with_description(title: Option<String>, description: String) -> Self {
        Self::Integer { title, description: Some(description) }
    }

    pub fn number_with_title(title: String, description: Option<String>, r#enum: Option<Vec<f64>>) -> Self {
        Self::Number { title: Some(title), description, r#enum }
    }

    pub fn number_with_description(title: Option<String>, description: String, r#enum: Option<Vec<f64>>) -> Self {
        Self::Number { title, description: Some(description), r#enum }
    }

    pub fn number_with_enum(title: Option<String>, description: Option<String>, r#enum: Vec<f64>) -> Self {
        Self::Number { title, description, r#enum: Some(r#enum) }
    }

    pub fn object_with_title(title: String, description: Option<String>, properties: Option<HashMap<String, Box<ContentSchema>>>, required: Option<Vec<String>>) -> Self {
        Self::Object { title: Some(title), description, properties, required }
    }

    pub fn object_with_description(title: Option<String>, description: String, properties: Option<HashMap<String, Box<ContentSchema>>>, required: Option<Vec<String>>) -> Self {
        Self::Object { title, description: Some(description), properties, required }
    }

    pub fn object_with_properties(title: Option<String>, description: Option<String>, properties: HashMap<String, Box<ContentSchema>>, required: Option<Vec<String>>) -> Self {
        Self::Object { title, description, properties: Some(properties), required }
    }

    pub fn object_with_required(title: Option<String>, description: Option<String>, properties: Option<HashMap<String, Box<ContentSchema>>>, required: Vec<String>) -> Self {
        Self::Object { title, description, properties, required: Some(required) }
    }

    pub fn string_with_title(title: String, description: Option<String>, r#enum: Option<Vec<String>>) -> Self {
        Self::r#String { title: Some(title), description, r#enum }
    }

    pub fn string_with_description(title: Option<String>, description: String, r#enum: Option<Vec<String>>) -> Self {
        Self::r#String { title, description: Some(description), r#enum }
    }

    pub fn string_with_enum(title: Option<String>, description: Option<String>, r#enum: Vec<String>) -> Self {
        Self::r#String { title, description, r#enum: Some(r#enum) }
    }

    pub fn video_with_title(title: String, description: Option<String>) -> Self {
        Self::Video { title: Some(title), description }
    }

    pub fn video_with_description(title: Option<String>, description: String) -> Self {
        Self::Video { title, description: Some(description) }
    }

    pub fn voice_with_title(title: String, description: Option<String>) -> Self {
        Self::Voice { title: Some(title), description }
    }

    pub fn voice_with_description(title: Option<String>, description: String) -> Self {
        Self::Voice { title, description: Some(description) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
