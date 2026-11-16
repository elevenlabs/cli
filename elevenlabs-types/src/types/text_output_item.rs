pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TextOutputItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix_ms: Option<i64>,
}

impl TextOutputItem {
    pub fn builder() -> TextOutputItemBuilder {
        <TextOutputItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextOutputItemBuilder {
    r#type: Option<String>,
    role: Option<String>,
    id: Option<String>,
    text: Option<String>,
    created_at_unix_ms: Option<i64>,
}

impl TextOutputItemBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn created_at_unix_ms(mut self, value: i64) -> Self {
        self.created_at_unix_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextOutputItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](TextOutputItemBuilder::role)
    /// - [`text`](TextOutputItemBuilder::text)
    pub fn build(self) -> Result<TextOutputItem, BuildError> {
        Ok(TextOutputItem {
            r#type: self.r#type,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            id: self.id,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            created_at_unix_ms: self.created_at_unix_ms,
        })
    }
}
