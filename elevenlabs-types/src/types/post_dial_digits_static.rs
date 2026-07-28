pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostDialDigitsStatic {
    /// DTMF digits to send after call connects (e.g., 'ww1234' for extension)
    #[serde(default)]
    pub value: String,
}

impl PostDialDigitsStatic {
    pub fn builder() -> PostDialDigitsStaticBuilder {
        <PostDialDigitsStaticBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostDialDigitsStaticBuilder {
    value: Option<String>,
}

impl PostDialDigitsStaticBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostDialDigitsStatic`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](PostDialDigitsStaticBuilder::value)
    pub fn build(self) -> Result<PostDialDigitsStatic, BuildError> {
        Ok(PostDialDigitsStatic {
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
