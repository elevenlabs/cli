pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TokenResponseModel {
    #[serde(default)]
    pub token: String,
}

impl TokenResponseModel {
    pub fn builder() -> TokenResponseModelBuilder {
        <TokenResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TokenResponseModelBuilder {
    token: Option<String>,
}

impl TokenResponseModelBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TokenResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](TokenResponseModelBuilder::token)
    pub fn build(self) -> Result<TokenResponseModel, BuildError> {
        Ok(TokenResponseModel {
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
