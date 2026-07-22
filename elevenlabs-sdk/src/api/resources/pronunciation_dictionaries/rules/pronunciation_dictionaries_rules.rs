use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct RulesClient {
    pub http_client: HttpClient,
}

impl RulesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Replaces all existing rules on the pronunciation dictionary with the provided ones.
    ///
    /// # Arguments
    ///
    /// * `pronunciation_dictionary_id` - The id of the pronunciation dictionary
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn set(
        &self,
        pronunciation_dictionary_id: &str,
        request: &BodySetRulesOnThePronunciationDictionaryV1PronunciationDictionariesPronunciationDictionaryIdSetRulesPost,
        options: Option<RequestOptions>,
    ) -> Result<PronunciationDictionaryRulesResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/pronunciation-dictionaries/{}/set-rules",
                    pronunciation_dictionary_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Add rules to the pronunciation dictionary. If a rule with the same string_to_replace already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `pronunciation_dictionary_id` - The id of the pronunciation dictionary
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add(
        &self,
        pronunciation_dictionary_id: &str,
        request: &PronunciationDictionary,
        options: Option<RequestOptions>,
    ) -> Result<PronunciationDictionaryRulesResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/pronunciation-dictionaries/{}/add-rules",
                    pronunciation_dictionary_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove rules from the pronunciation dictionary
    ///
    /// # Arguments
    ///
    /// * `pronunciation_dictionary_id` - The id of the pronunciation dictionary
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn remove(
        &self,
        pronunciation_dictionary_id: &str,
        request: &RemovePronunciationDictionaryRulesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PronunciationDictionaryRulesResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/pronunciation-dictionaries/{}/remove-rules",
                    pronunciation_dictionary_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
