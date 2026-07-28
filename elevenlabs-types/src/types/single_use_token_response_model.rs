pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SingleUseTokenResponseModel {
    /// A time bound single use token that expires after 15 minutes. Will be consumed on use.
    #[serde(default)]
    pub token: String,
}

impl SingleUseTokenResponseModel {
    pub fn builder() -> SingleUseTokenResponseModelBuilder {
        <SingleUseTokenResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SingleUseTokenResponseModelBuilder {
    token: Option<String>,
}

impl SingleUseTokenResponseModelBuilder {
    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SingleUseTokenResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token`](SingleUseTokenResponseModelBuilder::token)
    pub fn build(self) -> Result<SingleUseTokenResponseModel, BuildError> {
        Ok(SingleUseTokenResponseModel {
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
