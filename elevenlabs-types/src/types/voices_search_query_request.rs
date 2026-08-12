pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoicesSearchQueryRequest {
    /// The next page token to use for pagination. Returned from the previous request. Use this in combination with the has_more flag for reliable pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// How many voices to return at maximum. Can not exceed 100, defaults to 10. Page 0 may include more voices due to default voices being included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Search term to filter voices by. Searches in name, description, labels, category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Which field to sort by, one of 'created_at_unix' or 'name'. 'created_at_unix' may not be available for older voices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Which direction to sort the voices in. 'asc' or 'desc'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    /// Type of the voice to filter by. One of 'personal', 'community', 'default', 'workspace', 'non-default', 'non-community', 'saved'. 'non-default' is equal to all but 'default'. 'non-community' is equal to 'personal' and 'workspace' combined (excludes library copies). 'saved' is equal to non-default, but includes default voices if they have been added to a collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_type: Option<String>,
    /// Category of the voice to filter by. One of 'premade', 'cloned', 'generated', 'professional'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// State of the voice's fine tuning to filter by. Applicable only to professional voices clones. One of 'draft', 'not_verified', 'not_started', 'queued', 'fine_tuning', 'fine_tuned', 'failed', 'delayed'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_tuning_state: Option<String>,
    /// Collection ID to filter voices by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// Gender used for filtering, based on the voice's 'gender' label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Age used for filtering, based on the voice's 'age' label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    /// Languages used for filtering, based on the voice's 'language' label. Voices matching any of the given languages are returned.
    #[serde(default)]
    pub language: Vec<Option<String>>,
    /// Accent used for filtering, based on the voice's 'accent' label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    /// Use cases used for filtering, based on the voice's 'use_case' label. Voices matching any of the given use cases are returned.
    #[serde(default)]
    pub use_cases: Vec<Option<String>>,
    /// Filter to voices whose sharing notice period is at least the given number of days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_notice_period_days: Option<i64>,
    /// Whether to include voices that have a custom sharing rate. Defaults to including them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_custom_rates: Option<bool>,
    /// Whether to include voices that have live moderation enabled. Defaults to including them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_live_moderated: Option<bool>,
    /// When true, only return studio-quality voices (those whose category is 'high_quality').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_quality: Option<bool>,
    /// Whether to include the total count of voices found in the response. NOTE: The total_count value is a live snapshot and may change between requests as users create, modify, or delete voices. For pagination, rely on the has_more flag instead. Only enable this when you actually need the total count (e.g., for display purposes), as it incurs a performance cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_total_count: Option<bool>,
    /// Voice IDs to lookup by. Maximum 100 voice IDs.
    #[serde(default)]
    pub voice_ids: Vec<Option<String>>,
}

impl VoicesSearchQueryRequest {
    pub fn builder() -> VoicesSearchQueryRequestBuilder {
        <VoicesSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoicesSearchQueryRequestBuilder {
    next_page_token: Option<String>,
    page_size: Option<i64>,
    search: Option<String>,
    sort: Option<String>,
    sort_direction: Option<String>,
    voice_type: Option<String>,
    category: Option<String>,
    fine_tuning_state: Option<String>,
    collection_id: Option<String>,
    gender: Option<String>,
    age: Option<String>,
    language: Option<Vec<Option<String>>>,
    accent: Option<String>,
    use_cases: Option<Vec<Option<String>>>,
    min_notice_period_days: Option<i64>,
    include_custom_rates: Option<bool>,
    include_live_moderated: Option<bool>,
    high_quality: Option<bool>,
    include_total_count: Option<bool>,
    voice_ids: Option<Vec<Option<String>>>,
}

impl VoicesSearchQueryRequestBuilder {
    pub fn next_page_token(mut self, value: impl Into<String>) -> Self {
        self.next_page_token = Some(value.into());
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.sort = Some(value.into());
        self
    }

    pub fn sort_direction(mut self, value: impl Into<String>) -> Self {
        self.sort_direction = Some(value.into());
        self
    }

    pub fn voice_type(mut self, value: impl Into<String>) -> Self {
        self.voice_type = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn fine_tuning_state(mut self, value: impl Into<String>) -> Self {
        self.fine_tuning_state = Some(value.into());
        self
    }

    pub fn collection_id(mut self, value: impl Into<String>) -> Self {
        self.collection_id = Some(value.into());
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

    pub fn language(mut self, value: Vec<Option<String>>) -> Self {
        self.language = Some(value);
        self
    }

    pub fn accent(mut self, value: impl Into<String>) -> Self {
        self.accent = Some(value.into());
        self
    }

    pub fn use_cases(mut self, value: Vec<Option<String>>) -> Self {
        self.use_cases = Some(value);
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

    pub fn high_quality(mut self, value: bool) -> Self {
        self.high_quality = Some(value);
        self
    }

    pub fn include_total_count(mut self, value: bool) -> Self {
        self.include_total_count = Some(value);
        self
    }

    pub fn voice_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.voice_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoicesSearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language`](VoicesSearchQueryRequestBuilder::language)
    /// - [`use_cases`](VoicesSearchQueryRequestBuilder::use_cases)
    /// - [`voice_ids`](VoicesSearchQueryRequestBuilder::voice_ids)
    pub fn build(self) -> Result<VoicesSearchQueryRequest, BuildError> {
        Ok(VoicesSearchQueryRequest {
            next_page_token: self.next_page_token,
            page_size: self.page_size,
            search: self.search,
            sort: self.sort,
            sort_direction: self.sort_direction,
            voice_type: self.voice_type,
            category: self.category,
            fine_tuning_state: self.fine_tuning_state,
            collection_id: self.collection_id,
            gender: self.gender,
            age: self.age,
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            accent: self.accent,
            use_cases: self.use_cases.ok_or_else(|| BuildError::missing_field("use_cases"))?,
            min_notice_period_days: self.min_notice_period_days,
            include_custom_rates: self.include_custom_rates,
            include_live_moderated: self.include_live_moderated,
            high_quality: self.high_quality,
            include_total_count: self.include_total_count,
            voice_ids: self.voice_ids.ok_or_else(|| BuildError::missing_field("voice_ids"))?,
        })
    }
}

