pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExpiresAfter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    #[serde(default)]
    pub seconds: i64,
}

impl ExpiresAfter {
    pub fn builder() -> ExpiresAfterBuilder {
        <ExpiresAfterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpiresAfterBuilder {
    anchor: Option<String>,
    seconds: Option<i64>,
}

impl ExpiresAfterBuilder {
    pub fn anchor(mut self, value: impl Into<String>) -> Self {
        self.anchor = Some(value.into());
        self
    }

    pub fn seconds(mut self, value: i64) -> Self {
        self.seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpiresAfter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`seconds`](ExpiresAfterBuilder::seconds)
    pub fn build(self) -> Result<ExpiresAfter, BuildError> {
        Ok(ExpiresAfter {
            anchor: self.anchor,
            seconds: self.seconds.ok_or_else(|| BuildError::missing_field("seconds"))?,
        })
    }
}
