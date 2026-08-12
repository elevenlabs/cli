pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VerifyPvcVoiceCaptchaResponseModel {
    /// The status of the verify PVC captcha request. If the request was successful, the status will be 'ok'. Otherwise an error message with status 500 will be returned.
    #[serde(default)]
    pub status: String,
}

impl VerifyPvcVoiceCaptchaResponseModel {
    pub fn builder() -> VerifyPvcVoiceCaptchaResponseModelBuilder {
        <VerifyPvcVoiceCaptchaResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerifyPvcVoiceCaptchaResponseModelBuilder {
    status: Option<String>,
}

impl VerifyPvcVoiceCaptchaResponseModelBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VerifyPvcVoiceCaptchaResponseModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](VerifyPvcVoiceCaptchaResponseModelBuilder::status)
    pub fn build(self) -> Result<VerifyPvcVoiceCaptchaResponseModel, BuildError> {
        Ok(VerifyPvcVoiceCaptchaResponseModel {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
