use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod rules;
pub use rules::RulesClient;
pub struct PronunciationDictionariesClient {
    pub http_client: HttpClient,
    pub rules: RulesClient,
}

impl PronunciationDictionariesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            rules: RulesClient::new(config.clone())?,
        })
    }

    /// Creates a new pronunciation dictionary from a lexicon .PLS file
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
    ///         .pronunciation_dictionaries
    ///         .create_from_file(
    ///             &CreateFromFileRequest {
    ///                 file: b"test file content".to_vec(),
    ///                 name: "name".to_string(),
    ///                 description: None,
    ///                 workspace_access: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_from_file(
        &self,
        request: &CreateFromFileRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddPronunciationDictionaryResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/pronunciation-dictionaries/add-from-file",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Creates a new pronunciation dictionary from provided rules.
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
    ///     client.pronunciation_dictionaries.create_from_rules(&BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPost {
    ///         rules: vec![BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostRulesItem::Alias {
    ///             data: PronunciationDictionaryAliasRuleRequestModel {
    ///                 string_to_replace: "Thailand".to_string(),
    ///                 case_sensitive: Some(true),
    ///                 word_boundaries: Some(true),
    ///                 alias: "tie-land".to_string(),
    ///                 ..Default::default()
    ///             }
    ///         }],
    ///         name: "My Dictionary".to_string(),
    ///         description: None,
    ///         workspace_access: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn create_from_rules(
        &self,
        request: &BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPost,
        options: Option<RequestOptions>,
    ) -> Result<AddPronunciationDictionaryResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/pronunciation-dictionaries/add-from-rules",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get metadata for a pronunciation dictionary
    ///
    /// # Arguments
    ///
    /// * `pronunciation_dictionary_id` - The id of the pronunciation dictionary
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
    ///         .pronunciation_dictionaries
    ///         .get(&"21m00Tcm4TlvDq8ikWAM".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        pronunciation_dictionary_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPronunciationDictionaryWithRulesResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/pronunciation-dictionaries/{}",
                    pronunciation_dictionary_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Partially update the pronunciation dictionary without changing the version
    ///
    /// # Arguments
    ///
    /// * `pronunciation_dictionary_id` - The id of the pronunciation dictionary
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
    ///     client.pronunciation_dictionaries.update(&"21m00Tcm4TlvDq8ikWAM".to_string(), &BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIDPatch {
    ///         ..Default::default()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn update(
        &self,
        pronunciation_dictionary_id: &str,
        request: &BodyUpdatePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdPatch,
        options: Option<RequestOptions>,
    ) -> Result<GetPronunciationDictionaryMetadataResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/pronunciation-dictionaries/{}",
                    pronunciation_dictionary_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Get a PLS file with a pronunciation dictionary version rules
    ///
    /// # Arguments
    ///
    /// * `dictionary_id` - The id of the pronunciation dictionary
    /// * `version_id` - The id of the pronunciation dictionary version
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
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
    ///         .pronunciation_dictionaries
    ///         .download(
    ///             &"dictionary_id".to_string(),
    ///             &"version_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn download(
        &self,
        dictionary_id: &str,
        version_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!(
                    "v1/pronunciation-dictionaries/{}/{}/download",
                    dictionary_id, version_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Get a list of the pronunciation dictionaries you have access to and their metadata
    ///
    /// # Arguments
    ///
    /// * `cursor` - Used for fetching next page. Cursor is returned in the response.
    /// * `page_size` - How many pronunciation dictionaries to return at maximum. Can not exceed 100, defaults to 30.
    /// * `sort` - Which field to sort by, one of 'created_at_unix' or 'name'.
    /// * `sort_direction` - Which direction to sort the voices in. 'ascending' or 'descending'.
    /// * `include_archived` - Whether to include archived pronunciation dictionaries in the response.
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
    ///         .pronunciation_dictionaries
    ///         .list(
    ///             &PronunciationDictionariesListQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 sort: Some(PronunciationDictionariesListRequestSort::CreationTimeUnix),
    ///                 sort_direction: Some("sort_direction".to_string()),
    ///                 include_archived: Some(false),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PronunciationDictionariesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetPronunciationDictionariesMetadataResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/pronunciation-dictionaries",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("sort", request.sort.clone())
                    .string("sort_direction", request.sort_direction.clone())
                    .bool("include_archived", request.include_archived.clone())
                    .build(),
                options,
            )
            .await
    }
}
