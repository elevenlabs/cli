pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadLegalTerms {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl ReadLegalTerms {
    pub fn builder() -> ReadLegalTermsBuilder {
        <ReadLegalTermsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadLegalTermsBuilder {
    terms: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
}

impl ReadLegalTermsBuilder {
    pub fn terms(mut self, value: impl Into<String>) -> Self {
        self.terms = Some(value.into());
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadLegalTerms`].
    pub fn build(self) -> Result<ReadLegalTerms, BuildError> {
        Ok(ReadLegalTerms {
            terms: self.terms,
            start_date: self.start_date,
            end_date: self.end_date,
        })
    }
}
