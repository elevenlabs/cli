use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod settings;
pub use settings::SettingsClient3;
pub mod ivc;
pub use ivc::IvcClient;
pub mod pvc;
pub use pvc::PvcClient;
pub mod samples;
pub use samples::SamplesClient3;
pub struct VoicesClient {
    pub http_client: HttpClient,
    pub settings: SettingsClient3,
    pub ivc: IvcClient,
    pub pvc: PvcClient,
    pub samples: SamplesClient3,
}

impl VoicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            settings: SettingsClient3::new(config.clone())?,
            ivc: IvcClient::new(config.clone())?,
            pvc: PvcClient::new(config.clone())?,
            samples: SamplesClient3::new(config.clone())?,
        })
    }

    /// Returns a list of all available voices for a user. Stops working once the user's workspace exceeds 500 voices.
    ///
    /// # Arguments
    ///
    /// * `show_legacy` - If set to true, legacy premade voices will be included in responses from /v1/voices
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .voices
    ///         .get_all(
    ///             &GetAllQueryRequest {
    ///                 show_legacy: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_all(
        &self,
        request: &GetAllQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetVoicesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/voices",
                None,
                QueryBuilder::new()
                    .bool("show_legacy", request.show_legacy.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns metadata about a specific voice.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `with_settings` - This parameter is now deprecated. It is ignored and will be removed in a future version.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .voices
    ///         .get(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &VoicesGetQueryRequest {
    ///                 with_settings: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        voice_id: &str,
        request: &VoicesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Voice, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/voices/{}", voice_id),
                None,
                QueryBuilder::new()
                    .bool("with_settings", request.with_settings.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a voice by its ID.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .voices
    ///         .delete(&"21m00Tcm4TlvDq8ikWAM".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        voice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteVoiceResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/voices/{}", voice_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Edit a voice created by you.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .voices
    ///         .update(
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &UpdateRequest {
    ///                 files: vec![b"test file 1".to_vec(), b"test file 2".to_vec()],
    ///                 name: "name".to_string(),
    ///                 remove_background_noise: None,
    ///                 description: None,
    ///                 labels: None,
    ///                 moderate_metadata: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        voice_id: &str,
        request: &UpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<EditVoiceResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("v1/voices/{}/edit", voice_id),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Gets a list of all available voices for a user with search, filtering and pagination.
    ///
    /// # Arguments
    ///
    /// * `next_page_token` - The next page token to use for pagination. Returned from the previous request. Use this in combination with the has_more flag for reliable pagination.
    /// * `page_size` - How many voices to return at maximum. Can not exceed 100, defaults to 10. Page 0 may include more voices due to default voices being included.
    /// * `search` - Search term to filter voices by. Searches in name, description, labels, category.
    /// * `sort` - Which field to sort by, one of 'created_at_unix' or 'name'. 'created_at_unix' may not be available for older voices.
    /// * `sort_direction` - Which direction to sort the voices in. 'asc' or 'desc'.
    /// * `voice_type` - Type of the voice to filter by. One of 'personal', 'community', 'default', 'workspace', 'non-default', 'non-community', 'saved'. 'non-default' is equal to all but 'default'. 'non-community' is equal to 'personal' and 'workspace' combined (excludes library copies). 'saved' is equal to non-default, but includes default voices if they have been added to a collection.
    /// * `category` - Category of the voice to filter by. One of 'premade', 'cloned', 'generated', 'professional'
    /// * `fine_tuning_state` - State of the voice's fine tuning to filter by. Applicable only to professional voices clones. One of 'draft', 'not_verified', 'not_started', 'queued', 'fine_tuning', 'fine_tuned', 'failed', 'delayed'
    /// * `collection_id` - Collection ID to filter voices by.
    /// * `include_total_count` - Whether to include the total count of voices found in the response. NOTE: The total_count value is a live snapshot and may change between requests as users create, modify, or delete voices. For pagination, rely on the has_more flag instead. Only enable this when you actually need the total count (e.g., for display purposes), as it incurs a performance cost.
    /// * `voice_ids` - Voice IDs to lookup by. Maximum 100 voice IDs.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .voices
    ///         .search(
    ///             &VoicesSearchQueryRequest {
    ///                 next_page_token: Some("next_page_token".to_string()),
    ///                 page_size: Some(1),
    ///                 search: Some("search".to_string()),
    ///                 sort: Some("sort".to_string()),
    ///                 sort_direction: Some("sort_direction".to_string()),
    ///                 voice_type: Some("voice_type".to_string()),
    ///                 category: Some("category".to_string()),
    ///                 fine_tuning_state: Some("fine_tuning_state".to_string()),
    ///                 collection_id: Some("collection_id".to_string()),
    ///                 include_total_count: Some(true),
    ///                 voice_ids: vec![Some("voice_ids".to_string())],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        request: &VoicesSearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetVoicesV2Response, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v2/voices",
                None,
                QueryBuilder::new()
                    .string("next_page_token", request.next_page_token.clone())
                    .int("page_size", request.page_size.clone())
                    .string("search", request.search.clone())
                    .string("sort", request.sort.clone())
                    .string("sort_direction", request.sort_direction.clone())
                    .string("voice_type", request.voice_type.clone())
                    .string("category", request.category.clone())
                    .string("fine_tuning_state", request.fine_tuning_state.clone())
                    .string("collection_id", request.collection_id.clone())
                    .bool("include_total_count", request.include_total_count.clone())
                    .string_array("voice_ids", request.voice_ids.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a shared voice to your collection of Voices
    ///
    /// # Arguments
    ///
    /// * `public_user_id` - Public user ID used to publicly identify ElevenLabs users.
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .voices
    ///         .share(
    ///             &"63e06b7e7cafdc46be4d2e0b3f045940231ae058d508589653d74d1265a574ca".to_string(),
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             &BodyAddSharedVoiceV1VoicesAddPublicUserIDVoiceIDPost {
    ///                 new_name: "John Smith".to_string(),
    ///                 bookmarked: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn share(
        &self,
        public_user_id: &str,
        voice_id: &str,
        request: &BodyAddSharedVoiceV1VoicesAddPublicUserIdVoiceIdPost,
        options: Option<RequestOptions>,
    ) -> Result<AddVoiceResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/voices/add/{}/{}", public_user_id, voice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a list of shared voices.
    ///
    /// # Arguments
    ///
    /// * `page_size` - How many shared voices to return at maximum. Can not exceed 100, defaults to 30.
    /// * `category` - Voice category used for filtering
    /// * `gender` - Gender used for filtering
    /// * `age` - Age used for filtering
    /// * `accent` - Accent used for filtering
    /// * `language` - Language used for filtering
    /// * `locale` - Locale used for filtering
    /// * `search` - Search term used for filtering
    /// * `use_cases` - Use-case used for filtering
    /// * `descriptives` - Search term used for filtering
    /// * `featured` - Filter featured voices
    /// * `min_notice_period_days` - Filter voices with a minimum notice period of the given number of days.
    /// * `include_custom_rates` - Include/exclude voices with custom rates
    /// * `include_live_moderated` - Include/exclude voices that are live moderated
    /// * `reader_app_enabled` - Filter voices that are enabled for the reader app
    /// * `owner_id` - Filter voices by public owner ID
    /// * `sort` - Sort criteria. Must be one of: created_date, usage_character_count_1y, trending, cloned_by_count.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .voices
    ///         .get_shared(
    ///             &GetSharedQueryRequest {
    ///                 page_size: Some(1),
    ///                 category: Some(VoicesGetSharedRequestCategory::Professional),
    ///                 gender: Some("gender".to_string()),
    ///                 age: Some("age".to_string()),
    ///                 accent: Some("accent".to_string()),
    ///                 language: Some("language".to_string()),
    ///                 locale: Some("locale".to_string()),
    ///                 search: Some("search".to_string()),
    ///                 use_cases: vec![Some("use_cases".to_string())],
    ///                 descriptives: vec![Some("descriptives".to_string())],
    ///                 featured: Some(true),
    ///                 min_notice_period_days: Some(1),
    ///                 include_custom_rates: Some(true),
    ///                 include_live_moderated: Some(true),
    ///                 reader_app_enabled: Some(true),
    ///                 owner_id: Some("owner_id".to_string()),
    ///                 sort: Some(VoicesGetSharedRequestSort::CreatedDate),
    ///                 page: Some(1),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_shared(
        &self,
        request: &GetSharedQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLibraryVoicesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/shared-voices",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .serialize("category", request.category.clone())
                    .string("gender", request.gender.clone())
                    .string("age", request.age.clone())
                    .string("accent", request.accent.clone())
                    .string("language", request.language.clone())
                    .string("locale", request.locale.clone())
                    .string("search", request.search.clone())
                    .string_array("use_cases", request.use_cases.clone())
                    .string_array("descriptives", request.descriptives.clone())
                    .bool("featured", request.featured.clone())
                    .int(
                        "min_notice_period_days",
                        request.min_notice_period_days.clone(),
                    )
                    .bool("include_custom_rates", request.include_custom_rates.clone())
                    .bool(
                        "include_live_moderated",
                        request.include_live_moderated.clone(),
                    )
                    .bool("reader_app_enabled", request.reader_app_enabled.clone())
                    .string("owner_id", request.owner_id.clone())
                    .serialize("sort", request.sort.clone())
                    .int("page", request.page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a list of shared voices similar to the provided audio sample. If neither similarity_threshold nor top_k is provided, we will apply default values.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use elevenlabs_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ElevenlabsClient::new(config).expect("Failed to build client");
    ///     client
    ///         .voices
    ///         .find_similar_voices(
    ///             &FindSimilarVoicesRequest {
    ///                 audio_file: b"test file content".to_vec(),
    ///                 similarity_threshold: None,
    ///                 top_k: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn find_similar_voices(
        &self,
        request: &FindSimilarVoicesRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetLibraryVoicesResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/similar-voices",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }
}
