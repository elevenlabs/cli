pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidationError {
    #[serde(default)]
    pub loc: Vec<ValidationErrorLocItem>,
    #[serde(default)]
    pub msg: String,
    #[serde(default)]
    pub r#type: String,
}

impl ValidationError {
    pub fn builder() -> ValidationErrorBuilder {
        <ValidationErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidationErrorBuilder {
    loc: Option<Vec<ValidationErrorLocItem>>,
    msg: Option<String>,
    r#type: Option<String>,
}

impl ValidationErrorBuilder {
    pub fn loc(mut self, value: Vec<ValidationErrorLocItem>) -> Self {
        self.loc = Some(value);
        self
    }

    pub fn msg(mut self, value: impl Into<String>) -> Self {
        self.msg = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ValidationError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`loc`](ValidationErrorBuilder::loc)
    /// - [`msg`](ValidationErrorBuilder::msg)
    /// - [`r#type`](ValidationErrorBuilder::r#type)
    pub fn build(self) -> Result<ValidationError, BuildError> {
        Ok(ValidationError {
            loc: self.loc.ok_or_else(|| BuildError::missing_field("loc"))?,
            msg: self.msg.ok_or_else(|| BuildError::missing_field("msg"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
