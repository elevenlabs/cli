use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CollectionsClient {
    pub http_client: HttpClient,
}

impl CollectionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all the voice collections the user has access to, sorted by title.
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
    ///     client.voices.collections.list(None).await;
    /// }
    /// ```
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetUserVoiceCollectionsResponseModel, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/voices/collections", None, None, options)
            .await
    }

    /// Creates a new voice collection. The caller becomes its admin.
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
    ///         .collections
    ///         .create(
    ///             &BodyCreateVoiceCollectionV1VoicesCollectionsPost {
    ///                 title: "title".to_string(),
    ///                 icon: "icon".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &BodyCreateVoiceCollectionV1VoicesCollectionsPost,
        options: Option<RequestOptions>,
    ) -> Result<UserVoiceCollectionResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/voices/collections",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a voice collection the user has access to.
    ///
    /// # Arguments
    ///
    /// * `collection_id` - Collection ID.
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
    ///         .collections
    ///         .get(&"collection_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        collection_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<UserVoiceCollectionResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/voices/collections/{}", collection_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a voice collection. The voices in it are not deleted. Requires admin access to the collection.
    ///
    /// # Arguments
    ///
    /// * `collection_id` - Collection ID.
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
    ///         .collections
    ///         .delete(&"collection_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        collection_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteUserVoiceCollectionResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/voices/collections/{}", collection_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates the title and icon of a voice collection. Fields that are omitted are left unchanged. Requires editor access to the collection.
    ///
    /// # Arguments
    ///
    /// * `collection_id` - Collection ID.
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
    ///         .collections
    ///         .update(
    ///             &"collection_id".to_string(),
    ///             &BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIDPatch {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        collection_id: &str,
        request: &BodyUpdateVoiceCollectionV1VoicesCollectionsCollectionIdPatch,
        options: Option<RequestOptions>,
    ) -> Result<UserVoiceCollectionResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/voices/collections/{}", collection_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds a voice to a voice collection. Adding a voice that is already in the collection succeeds without changes. Requires editor access to the collection.
    ///
    /// # Arguments
    ///
    /// * `collection_id` - Collection ID.
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
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
    ///         .collections
    ///         .add_voice(
    ///             &"collection_id".to_string(),
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_voice(
        &self,
        collection_id: &str,
        voice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AddVoiceToUserVoiceCollectionResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/voices/collections/{}/voice/{}", collection_id, voice_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Removes a voice from a voice collection. The voice itself is not deleted. Requires editor access to the collection.
    ///
    /// # Arguments
    ///
    /// * `collection_id` - Collection ID.
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
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
    ///         .collections
    ///         .remove_voice(
    ///             &"collection_id".to_string(),
    ///             &"21m00Tcm4TlvDq8ikWAM".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn remove_voice(
        &self,
        collection_id: &str,
        voice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RemoveVoiceFromUserVoiceCollectionResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/voices/collections/{}/voice/{}", collection_id, voice_id),
                None,
                None,
                options,
            )
            .await
    }
}
