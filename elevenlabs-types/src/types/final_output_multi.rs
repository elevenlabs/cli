pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Server payload indicating the final output for a specific context.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FinalOutputMulti {
    /// Indicates this is the final message for the context.
    #[serde(rename = "isFinal")]
    pub is_final: bool,
    /// The context_id for which this is the final message.
    #[serde(rename = "contextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl FinalOutputMulti {
    pub fn builder() -> FinalOutputMultiBuilder {
        <FinalOutputMultiBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinalOutputMultiBuilder {
    is_final: Option<bool>,
    context_id: Option<String>,
}

impl FinalOutputMultiBuilder {
    pub fn is_final(mut self, value: bool) -> Self {
        self.is_final = Some(value);
        self
    }

    pub fn context_id(mut self, value: impl Into<String>) -> Self {
        self.context_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FinalOutputMulti`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_final`](FinalOutputMultiBuilder::is_final)
    pub fn build(self) -> Result<FinalOutputMulti, BuildError> {
        Ok(FinalOutputMulti {
            is_final: self.is_final.ok_or_else(|| BuildError::missing_field("is_final"))?,
            context_id: self.context_id,
        })
    }
}
