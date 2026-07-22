pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "auth_type")]
#[non_exhaustive]
pub enum ListAuthConnectionsResponseAuthConnectionsItem {
        #[serde(rename = "api_integration_oauth2_auth_code")]
        #[non_exhaustive]
        ApiIntegrationOauth2AuthCode {
            #[serde(flatten)]
            data: ApiIntegrationOAuth2AuthCodeResponse,
        },

        #[serde(rename = "api_integration_oauth2_custom_app")]
        #[non_exhaustive]
        ApiIntegrationOauth2CustomApp {
            #[serde(flatten)]
            data: ApiIntegrationOAuth2CustomAppResponse,
        },

        #[serde(rename = "basic_auth")]
        #[non_exhaustive]
        BasicAuth {
            #[serde(flatten)]
            data: BasicAuthResponse,
        },

        #[serde(rename = "bearer_auth")]
        #[non_exhaustive]
        BearerAuth {
            #[serde(flatten)]
            data: BearerAuthResponse,
        },

        #[serde(rename = "custom_header_auth")]
        #[non_exhaustive]
        CustomHeaderAuth {
            #[serde(flatten)]
            data: CustomHeaderAuthResponse,
        },

        #[serde(rename = "mtls")]
        #[non_exhaustive]
        Mtls {
            #[serde(flatten)]
            data: MtlsAuthResponse,
        },

        #[serde(rename = "oauth2_client_credentials")]
        #[non_exhaustive]
        Oauth2ClientCredentials {
            #[serde(flatten)]
            data: OAuth2ClientCredsResponse,
        },

        #[serde(rename = "oauth2_jwt")]
        #[non_exhaustive]
        Oauth2Jwt {
            #[serde(flatten)]
            data: OAuth2JwtResponse,
        },

        #[serde(rename = "private_key_jwt")]
        #[non_exhaustive]
        PrivateKeyJwt {
            #[serde(flatten)]
            data: PrivateKeyJwtResponse,
        },

        #[serde(rename = "refresh_token_auth")]
        #[non_exhaustive]
        RefreshTokenAuth {
            #[serde(flatten)]
            data: RefreshTokenAuthResponse,
        },

        #[serde(rename = "slack_bot_auth")]
        #[non_exhaustive]
        SlackBotAuth {
            #[serde(flatten)]
            data: SlackBotAuthResponse,
        },

        #[serde(rename = "url_secret")]
        #[non_exhaustive]
        UrlSecret {
            #[serde(flatten)]
            data: UrlSecretAuthResponse,
        },

        #[serde(rename = "whatsapp_auth")]
        #[non_exhaustive]
        WhatsappAuth {
            #[serde(flatten)]
            data: WhatsAppAuthResponse,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ListAuthConnectionsResponseAuthConnectionsItem {
    pub fn api_integration_oauth2auth_code(data: ApiIntegrationOAuth2AuthCodeResponse) -> Self {
        Self::ApiIntegrationOauth2AuthCode { data }
    }

    pub fn api_integration_oauth2custom_app(data: ApiIntegrationOAuth2CustomAppResponse) -> Self {
        Self::ApiIntegrationOauth2CustomApp { data }
    }

    pub fn basic_auth(data: BasicAuthResponse) -> Self {
        Self::BasicAuth { data }
    }

    pub fn bearer_auth(data: BearerAuthResponse) -> Self {
        Self::BearerAuth { data }
    }

    pub fn custom_header_auth(data: CustomHeaderAuthResponse) -> Self {
        Self::CustomHeaderAuth { data }
    }

    pub fn mtls(data: MtlsAuthResponse) -> Self {
        Self::Mtls { data }
    }

    pub fn oauth2client_credentials(data: OAuth2ClientCredsResponse) -> Self {
        Self::Oauth2ClientCredentials { data }
    }

    pub fn oauth2jwt(data: OAuth2JwtResponse) -> Self {
        Self::Oauth2Jwt { data }
    }

    pub fn private_key_jwt(data: PrivateKeyJwtResponse) -> Self {
        Self::PrivateKeyJwt { data }
    }

    pub fn refresh_token_auth(data: RefreshTokenAuthResponse) -> Self {
        Self::RefreshTokenAuth { data }
    }

    pub fn slack_bot_auth(data: SlackBotAuthResponse) -> Self {
        Self::SlackBotAuth { data }
    }

    pub fn url_secret(data: UrlSecretAuthResponse) -> Self {
        Self::UrlSecret { data }
    }

    pub fn whatsapp_auth(data: WhatsAppAuthResponse) -> Self {
        Self::WhatsappAuth { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
