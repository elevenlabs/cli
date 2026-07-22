pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VoiceSharingModerationCheckResponseModel {
    /// The date the moderation check was made in Unix time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_checked_unix: Option<i64>,
    /// The name value of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_value: Option<String>,
    /// Whether the name check was successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_check: Option<bool>,
    /// The description value of the voice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_value: Option<String>,
    /// Whether the description check was successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_check: Option<bool>,
    /// A list of sample IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_ids: Option<Vec<String>>,
    /// A list of sample checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_checks: Option<Vec<f64>>,
    /// A list of captcha IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_ids: Option<Vec<String>>,
    /// A list of CAPTCHA check values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_checks: Option<Vec<f64>>,
}

impl VoiceSharingModerationCheckResponseModel {
    pub fn builder() -> VoiceSharingModerationCheckResponseModelBuilder {
        <VoiceSharingModerationCheckResponseModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceSharingModerationCheckResponseModelBuilder {
    date_checked_unix: Option<i64>,
    name_value: Option<String>,
    name_check: Option<bool>,
    description_value: Option<String>,
    description_check: Option<bool>,
    sample_ids: Option<Vec<String>>,
    sample_checks: Option<Vec<f64>>,
    captcha_ids: Option<Vec<String>>,
    captcha_checks: Option<Vec<f64>>,
}

impl VoiceSharingModerationCheckResponseModelBuilder {
    pub fn date_checked_unix(mut self, value: i64) -> Self {
        self.date_checked_unix = Some(value);
        self
    }

    pub fn name_value(mut self, value: impl Into<String>) -> Self {
        self.name_value = Some(value.into());
        self
    }

    pub fn name_check(mut self, value: bool) -> Self {
        self.name_check = Some(value);
        self
    }

    pub fn description_value(mut self, value: impl Into<String>) -> Self {
        self.description_value = Some(value.into());
        self
    }

    pub fn description_check(mut self, value: bool) -> Self {
        self.description_check = Some(value);
        self
    }

    pub fn sample_ids(mut self, value: Vec<String>) -> Self {
        self.sample_ids = Some(value);
        self
    }

    pub fn sample_checks(mut self, value: Vec<f64>) -> Self {
        self.sample_checks = Some(value);
        self
    }

    pub fn captcha_ids(mut self, value: Vec<String>) -> Self {
        self.captcha_ids = Some(value);
        self
    }

    pub fn captcha_checks(mut self, value: Vec<f64>) -> Self {
        self.captcha_checks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoiceSharingModerationCheckResponseModel`].
    pub fn build(self) -> Result<VoiceSharingModerationCheckResponseModel, BuildError> {
        Ok(VoiceSharingModerationCheckResponseModel {
            date_checked_unix: self.date_checked_unix,
            name_value: self.name_value,
            name_check: self.name_check,
            description_value: self.description_value,
            description_check: self.description_check,
            sample_ids: self.sample_ids,
            sample_checks: self.sample_checks,
            captcha_ids: self.captcha_ids,
            captcha_checks: self.captcha_checks,
        })
    }
}
