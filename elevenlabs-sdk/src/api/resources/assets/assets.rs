use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AssetsClient {
    pub http_client: HttpClient,
}

impl AssetsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List assets in the workspace, most recently created first.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Number of assets to return.
    /// * `cursor` - Token from a previous response's `next_cursor`. Omit to fetch the first page.
    /// * `search` - Optional free-text search filter over asset names.
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
    ///         .assets
    ///         .list(
    ///             &AssetsListQueryRequest {
    ///                 page_size: Some(1),
    ///                 cursor: Some("cursor".to_string()),
    ///                 search: Some("search".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AssetsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AssetListResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/assets",
                None,
                QueryBuilder::new()
                    .int("page_size", request.page_size.clone())
                    .string("cursor", request.cursor.clone())
                    .string("search", request.search.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Upload a new asset.
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
    ///         .assets
    ///         .create(
    ///             &CreateRequest {
    ///                 asset: b"test file content".to_vec(),
    ///                 name: "name".to_string(),
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
    ) -> Result<AssetResponse, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "v1/assets",
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Retrieve a single asset by ID.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - ID of the asset.
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
    ///         .assets
    ///         .get(&"5xM2KqOnZyce22SPZ9d4".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AssetResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/assets/{}", asset_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete an asset by ID.
    ///
    /// # Arguments
    ///
    /// * `asset_id` - ID of the asset.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .assets
    ///         .delete(&"5xM2KqOnZyce22SPZ9d4".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        asset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/assets/{}", asset_id),
                None,
                None,
                options,
            )
            .await
    }
}
