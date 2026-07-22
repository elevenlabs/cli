use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SettingsClient3 {
    pub http_client: HttpClient,
}

impl SettingsClient3 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Gets the default settings for voices. "similarity_boost" corresponds to"Clarity + Similarity Enhancement" in the web app and "stability" corresponds to "Stability" slider in the web app.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_default(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<VoiceSettings, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/voices/settings/default",
                None,
                None,
                options,
            )
            .await
    }

    /// Returns the settings for a specific voice. "similarity_boost" corresponds to"Clarity + Similarity Enhancement" in the web app and "stability" corresponds to "Stability" slider in the web app.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        voice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<VoiceSettings, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/voices/{}/settings", voice_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Edit your settings for a specific voice. "similarity_boost" corresponds to "Clarity + Similarity Enhancement" in the web app and "stability" corresponds to "Stability" slider in the web app.
    ///
    /// # Arguments
    ///
    /// * `voice_id` - ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices.
    /// * `request` - The settings for a specific voice.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        voice_id: &str,
        request: &VoiceSettings,
        options: Option<RequestOptions>,
    ) -> Result<EditVoiceSettingsResponseModel, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/voices/{}/settings/edit", voice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
