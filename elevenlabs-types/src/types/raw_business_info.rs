pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Business profile data extracted from a single page.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RawBusinessInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<LenientOptionalStr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<LenientOptionalStr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_phone: Option<LenientOptionalStr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<RawLocationInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_country: Option<LenientOptionalStr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_state: Option<LenientOptionalStr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_currency: Option<LenientOptionalStr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_language: Option<LenientOptionalStr>,
}

impl RawBusinessInfo {
    pub fn builder() -> RawBusinessInfoBuilder {
        <RawBusinessInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RawBusinessInfoBuilder {
    company_name: Option<LenientOptionalStr>,
    description: Option<LenientOptionalStr>,
    business_phone: Option<LenientOptionalStr>,
    locations: Option<Vec<RawLocationInfo>>,
    business_country: Option<LenientOptionalStr>,
    business_state: Option<LenientOptionalStr>,
    suggested_currency: Option<LenientOptionalStr>,
    suggested_language: Option<LenientOptionalStr>,
}

impl RawBusinessInfoBuilder {
    pub fn company_name(mut self, value: LenientOptionalStr) -> Self {
        self.company_name = Some(value);
        self
    }

    pub fn description(mut self, value: LenientOptionalStr) -> Self {
        self.description = Some(value);
        self
    }

    pub fn business_phone(mut self, value: LenientOptionalStr) -> Self {
        self.business_phone = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<RawLocationInfo>) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn business_country(mut self, value: LenientOptionalStr) -> Self {
        self.business_country = Some(value);
        self
    }

    pub fn business_state(mut self, value: LenientOptionalStr) -> Self {
        self.business_state = Some(value);
        self
    }

    pub fn suggested_currency(mut self, value: LenientOptionalStr) -> Self {
        self.suggested_currency = Some(value);
        self
    }

    pub fn suggested_language(mut self, value: LenientOptionalStr) -> Self {
        self.suggested_language = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RawBusinessInfo`].
    pub fn build(self) -> Result<RawBusinessInfo, BuildError> {
        Ok(RawBusinessInfo {
            company_name: self.company_name,
            description: self.description,
            business_phone: self.business_phone,
            locations: self.locations,
            business_country: self.business_country,
            business_state: self.business_state,
            suggested_currency: self.suggested_currency,
            suggested_language: self.suggested_language,
        })
    }
}
