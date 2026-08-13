pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_shared
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSharedQueryRequest {
    /// How many shared voices to return at maximum. Can not exceed 100, defaults to 30.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Voice category used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<VoicesGetSharedRequestCategory>,
    /// Gender used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Age used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    /// Accent used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    /// Language used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Locale used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Search term used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Use-case used for filtering
    #[serde(default)]
    pub use_cases: Vec<Option<String>>,
    /// Search term used for filtering
    #[serde(default)]
    pub descriptives: Vec<Option<String>>,
    /// Filter featured voices
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured: Option<bool>,
    /// Filter voices with a minimum notice period of the given number of days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_notice_period_days: Option<i64>,
    /// Include/exclude voices with custom rates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_custom_rates: Option<bool>,
    /// Include/exclude voices that are live moderated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_live_moderated: Option<bool>,
    /// Filter voices that are enabled for the reader app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_app_enabled: Option<bool>,
    /// Filter voices by public owner ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// Sort criteria. Must be one of: created_date, usage_character_count_1y, trending, cloned_by_count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<VoicesGetSharedRequestSort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl GetSharedQueryRequest {
    pub fn builder() -> GetSharedQueryRequestBuilder {
        <GetSharedQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSharedQueryRequestBuilder {
    page_size: Option<i64>,
    category: Option<VoicesGetSharedRequestCategory>,
    gender: Option<String>,
    age: Option<String>,
    accent: Option<String>,
    language: Option<String>,
    locale: Option<String>,
    search: Option<String>,
    use_cases: Option<Vec<Option<String>>>,
    descriptives: Option<Vec<Option<String>>>,
    featured: Option<bool>,
    min_notice_period_days: Option<i64>,
    include_custom_rates: Option<bool>,
    include_live_moderated: Option<bool>,
    reader_app_enabled: Option<bool>,
    owner_id: Option<String>,
    sort: Option<VoicesGetSharedRequestSort>,
    page: Option<i64>,
}

impl GetSharedQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn category(mut self, value: VoicesGetSharedRequestCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(value.into());
        self
    }

    pub fn age(mut self, value: impl Into<String>) -> Self {
        self.age = Some(value.into());
        self
    }

    pub fn accent(mut self, value: impl Into<String>) -> Self {
        self.accent = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn use_cases(mut self, value: Vec<Option<String>>) -> Self {
        self.use_cases = Some(value);
        self
    }

    pub fn descriptives(mut self, value: Vec<Option<String>>) -> Self {
        self.descriptives = Some(value);
        self
    }

    pub fn featured(mut self, value: bool) -> Self {
        self.featured = Some(value);
        self
    }

    pub fn min_notice_period_days(mut self, value: i64) -> Self {
        self.min_notice_period_days = Some(value);
        self
    }

    pub fn include_custom_rates(mut self, value: bool) -> Self {
        self.include_custom_rates = Some(value);
        self
    }

    pub fn include_live_moderated(mut self, value: bool) -> Self {
        self.include_live_moderated = Some(value);
        self
    }

    pub fn reader_app_enabled(mut self, value: bool) -> Self {
        self.reader_app_enabled = Some(value);
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn sort(mut self, value: VoicesGetSharedRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSharedQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`use_cases`](GetSharedQueryRequestBuilder::use_cases)
    /// - [`descriptives`](GetSharedQueryRequestBuilder::descriptives)
    pub fn build(self) -> Result<GetSharedQueryRequest, BuildError> {
        Ok(GetSharedQueryRequest {
            page_size: self.page_size,
            category: self.category,
            gender: self.gender,
            age: self.age,
            accent: self.accent,
            language: self.language,
            locale: self.locale,
            search: self.search,
            use_cases: self.use_cases.ok_or_else(|| BuildError::missing_field("use_cases"))?,
            descriptives: self.descriptives.ok_or_else(|| BuildError::missing_field("descriptives"))?,
            featured: self.featured,
            min_notice_period_days: self.min_notice_period_days,
            include_custom_rates: self.include_custom_rates,
            include_live_moderated: self.include_live_moderated,
            reader_app_enabled: self.reader_app_enabled,
            owner_id: self.owner_id,
            sort: self.sort,
            page: self.page,
        })
    }
}

