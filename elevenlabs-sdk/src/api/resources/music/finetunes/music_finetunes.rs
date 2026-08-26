use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FinetunesClient {
    pub http_client: HttpClient,
}

impl FinetunesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List music finetunes accessible to you (your own, workspace-shared, and ElevenLabs-curated), with optional filtering, sorting, and cursor pagination.
    ///
    /// # Arguments
    ///
    /// * `cursor` - Used for fetching the next page. Cursor is returned in the response.
    /// * `page_size` - How many finetunes to return. Max 150, default 50.
    /// * `visibility` - Filter by visibility. 'private' returns private finetunes; 'workspace' returns workspace-shared finetunes; 'public' returns public finetunes, which are currently ElevenLabs curated finetunes. Omit to return all accessible finetunes.
    /// * `created_by` - Filter by creator. 'self' returns finetunes you created; 'workspace' returns finetunes created by workspace teammates; 'elevenlabs' returns ElevenLabs curated finetunes. Omit to return finetunes from all creators.
    /// * `sort` - Sort by field (created_at or name)
    /// * `sort_direction` - Sort direction (asc or desc)
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
    ///         .music
    ///         .finetunes
    ///         .list(
    ///             &MusicFinetunesListQueryRequest {
    ///                 cursor: Some("cursor".to_string()),
    ///                 page_size: Some(1),
    ///                 visibility: Some(FinetuneVisibility::Private),
    ///                 created_by: Some(FinetuneCreatedBy::Self),
    ///                 sort: Some(FinetunesListRequestSort::CreatedAt),
    ///                 sort_direction: Some(FinetunesListRequestSortDirection::Asc),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &MusicFinetunesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MusicFinetunePageResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/music/finetunes",
                None,
                QueryBuilder::new()
                    .string("cursor", request.cursor.clone())
                    .int("page_size", request.page_size.clone())
                    .serialize("visibility", request.visibility.clone())
                    .serialize("created_by", request.created_by.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("sort_direction", request.sort_direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new music finetune
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
    ///         .music
    ///         .finetunes
    ///         .create(
    ///             &CreateRequest {
    ///                 files: vec![b"test file 1".to_vec(), b"test file 2".to_vec()],
    ///                 name: "name".to_string(),
    ///                 primary_genre: "primary_genre".to_string(),
    ///                 tags: None,
    ///                 visibility: None,
    ///                 model_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<MusicFinetuneResponseModel, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/music/finetunes",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Get a music finetune.
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
    ///         .music
    ///         .finetunes
    ///         .get(&"finetune_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        finetune_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MusicFinetuneResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/music/finetunes/{}", finetune_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a music finetune
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
    ///         .music
    ///         .finetunes
    ///         .delete(&"finetune_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        finetune_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<MusicFinetuneResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/music/finetunes/{}", finetune_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a music finetune.
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
    ///         .music
    ///         .finetunes
    ///         .update(
    ///             &"finetune_id".to_string(),
    ///             &UpdateMusicFinetuneRequestModel {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        finetune_id: &str,
        request: &UpdateMusicFinetuneRequestModel,
        options: Option<RequestOptions>,
    ) -> Result<MusicFinetuneResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/music/finetunes/{}", finetune_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
