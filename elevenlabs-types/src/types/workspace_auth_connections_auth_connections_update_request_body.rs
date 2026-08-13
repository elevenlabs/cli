pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "auth_type")]
#[non_exhaustive]
pub enum AuthConnectionsUpdateRequestBody {
        #[serde(rename = "oauth2_client_credentials")]
        #[non_exhaustive]
        Oauth2ClientCredentials {
            #[serde(skip_serializing_if = "Option::is_none")]
            provider: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            scopes: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            extra_params: Option<HashMap<String, Option<String>>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            basic_auth_in_header: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            client_secret: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            custom_headers: Option<HashMap<String, Option<String>>>,
        },

        #[serde(rename = "basic_auth")]
        #[non_exhaustive]
        BasicAuth {
            #[serde(skip_serializing_if = "Option::is_none")]
            provider: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            username: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            password: Option<String>,
        },

        #[serde(rename = "bearer_auth")]
        #[non_exhaustive]
        BearerAuth {
            #[serde(skip_serializing_if = "Option::is_none")]
            provider: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            token: Option<String>,
        },

        #[serde(rename = "oauth2_jwt")]
        #[non_exhaustive]
        Oauth2Jwt {
            #[serde(skip_serializing_if = "Option::is_none")]
            provider: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>,
            #[serde(skip_serializing_if = "Option::is_none")]
            key_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            issuer: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            audience: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            subject: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            expiration_seconds: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            extra_params: Option<HashMap<String, Option<String>>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            scopes: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>,
            #[serde(skip_serializing_if = "Option::is_none")]
            secret_key: Option<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AuthConnectionsUpdateRequestBody {
    pub fn oauth2client_credentials() -> Self {
        Self::Oauth2ClientCredentials { provider: None, client_id: None, scopes: None, extra_params: None, basic_auth_in_header: None, client_secret: None, custom_headers: None }
    }

    pub fn basic_auth() -> Self {
        Self::BasicAuth { provider: None, username: None, password: None }
    }

    pub fn bearer_auth() -> Self {
        Self::BearerAuth { provider: None, token: None }
    }

    pub fn oauth2jwt() -> Self {
        Self::Oauth2Jwt { provider: None, algorithm: None, key_id: None, issuer: None, audience: None, subject: None, expiration_seconds: None, extra_params: None, scopes: None, token_response_field: None, secret_key: None }
    }

    pub fn oauth2client_credentials_with_provider(provider: String, client_id: Option<String>, scopes: Option<Vec<String>>, extra_params: Option<HashMap<String, Option<String>>>, basic_auth_in_header: Option<bool>, client_secret: Option<String>, custom_headers: Option<HashMap<String, Option<String>>>) -> Self {
        Self::Oauth2ClientCredentials { provider: Some(provider), client_id, scopes, extra_params, basic_auth_in_header, client_secret, custom_headers }
    }

    pub fn oauth2client_credentials_with_client_id(provider: Option<String>, client_id: String, scopes: Option<Vec<String>>, extra_params: Option<HashMap<String, Option<String>>>, basic_auth_in_header: Option<bool>, client_secret: Option<String>, custom_headers: Option<HashMap<String, Option<String>>>) -> Self {
        Self::Oauth2ClientCredentials { provider, client_id: Some(client_id), scopes, extra_params, basic_auth_in_header, client_secret, custom_headers }
    }

    pub fn oauth2client_credentials_with_scopes(provider: Option<String>, client_id: Option<String>, scopes: Vec<String>, extra_params: Option<HashMap<String, Option<String>>>, basic_auth_in_header: Option<bool>, client_secret: Option<String>, custom_headers: Option<HashMap<String, Option<String>>>) -> Self {
        Self::Oauth2ClientCredentials { provider, client_id, scopes: Some(scopes), extra_params, basic_auth_in_header, client_secret, custom_headers }
    }

    pub fn oauth2client_credentials_with_extra_params(provider: Option<String>, client_id: Option<String>, scopes: Option<Vec<String>>, extra_params: HashMap<String, Option<String>>, basic_auth_in_header: Option<bool>, client_secret: Option<String>, custom_headers: Option<HashMap<String, Option<String>>>) -> Self {
        Self::Oauth2ClientCredentials { provider, client_id, scopes, extra_params: Some(extra_params), basic_auth_in_header, client_secret, custom_headers }
    }

    pub fn oauth2client_credentials_with_basic_auth_in_header(provider: Option<String>, client_id: Option<String>, scopes: Option<Vec<String>>, extra_params: Option<HashMap<String, Option<String>>>, basic_auth_in_header: bool, client_secret: Option<String>, custom_headers: Option<HashMap<String, Option<String>>>) -> Self {
        Self::Oauth2ClientCredentials { provider, client_id, scopes, extra_params, basic_auth_in_header: Some(basic_auth_in_header), client_secret, custom_headers }
    }

    pub fn oauth2client_credentials_with_client_secret(provider: Option<String>, client_id: Option<String>, scopes: Option<Vec<String>>, extra_params: Option<HashMap<String, Option<String>>>, basic_auth_in_header: Option<bool>, client_secret: String, custom_headers: Option<HashMap<String, Option<String>>>) -> Self {
        Self::Oauth2ClientCredentials { provider, client_id, scopes, extra_params, basic_auth_in_header, client_secret: Some(client_secret), custom_headers }
    }

    pub fn oauth2client_credentials_with_custom_headers(provider: Option<String>, client_id: Option<String>, scopes: Option<Vec<String>>, extra_params: Option<HashMap<String, Option<String>>>, basic_auth_in_header: Option<bool>, client_secret: Option<String>, custom_headers: HashMap<String, Option<String>>) -> Self {
        Self::Oauth2ClientCredentials { provider, client_id, scopes, extra_params, basic_auth_in_header, client_secret, custom_headers: Some(custom_headers) }
    }

    pub fn basic_auth_with_provider(provider: String, username: Option<String>, password: Option<String>) -> Self {
        Self::BasicAuth { provider: Some(provider), username, password }
    }

    pub fn basic_auth_with_username(provider: Option<String>, username: String, password: Option<String>) -> Self {
        Self::BasicAuth { provider, username: Some(username), password }
    }

    pub fn basic_auth_with_password(provider: Option<String>, username: Option<String>, password: String) -> Self {
        Self::BasicAuth { provider, username, password: Some(password) }
    }

    pub fn bearer_auth_with_provider(provider: String, token: Option<String>) -> Self {
        Self::BearerAuth { provider: Some(provider), token }
    }

    pub fn bearer_auth_with_token(provider: Option<String>, token: String) -> Self {
        Self::BearerAuth { provider, token: Some(token) }
    }

    pub fn oauth2jwt_with_provider(provider: String, algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: Option<String>, audience: Option<String>, subject: Option<String>, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, Option<String>>>, scopes: Option<Vec<String>>, token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>, secret_key: Option<String>) -> Self {
        Self::Oauth2Jwt { provider: Some(provider), algorithm, key_id, issuer, audience, subject, expiration_seconds, extra_params, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_algorithm(provider: Option<String>, algorithm: UpdateOAuth2JwtRequestAlgorithm, key_id: Option<String>, issuer: Option<String>, audience: Option<String>, subject: Option<String>, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, Option<String>>>, scopes: Option<Vec<String>>, token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>, secret_key: Option<String>) -> Self {
        Self::Oauth2Jwt { provider, algorithm: Some(algorithm), key_id, issuer, audience, subject, expiration_seconds, extra_params, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_key_id(provider: Option<String>, algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>, key_id: String, issuer: Option<String>, audience: Option<String>, subject: Option<String>, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, Option<String>>>, scopes: Option<Vec<String>>, token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>, secret_key: Option<String>) -> Self {
        Self::Oauth2Jwt { provider, algorithm, key_id: Some(key_id), issuer, audience, subject, expiration_seconds, extra_params, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_issuer(provider: Option<String>, algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: String, audience: Option<String>, subject: Option<String>, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, Option<String>>>, scopes: Option<Vec<String>>, token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>, secret_key: Option<String>) -> Self {
        Self::Oauth2Jwt { provider, algorithm, key_id, issuer: Some(issuer), audience, subject, expiration_seconds, extra_params, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_audience(provider: Option<String>, algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: Option<String>, audience: String, subject: Option<String>, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, Option<String>>>, scopes: Option<Vec<String>>, token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>, secret_key: Option<String>) -> Self {
        Self::Oauth2Jwt { provider, algorithm, key_id, issuer, audience: Some(audience), subject, expiration_seconds, extra_params, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_subject(provider: Option<String>, algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: Option<String>, audience: Option<String>, subject: String, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, Option<String>>>, scopes: Option<Vec<String>>, token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>, secret_key: Option<String>) -> Self {
        Self::Oauth2Jwt { provider, algorithm, key_id, issuer, audience, subject: Some(subject), expiration_seconds, extra_params, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_expiration_seconds(provider: Option<String>, algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: Option<String>, audience: Option<String>, subject: Option<String>, expiration_seconds: i64, extra_params: Option<HashMap<String, Option<String>>>, scopes: Option<Vec<String>>, token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>, secret_key: Option<String>) -> Self {
        Self::Oauth2Jwt { provider, algorithm, key_id, issuer, audience, subject, expiration_seconds: Some(expiration_seconds), extra_params, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_extra_params(provider: Option<String>, algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: Option<String>, audience: Option<String>, subject: Option<String>, expiration_seconds: Option<i64>, extra_params: HashMap<String, Option<String>>, scopes: Option<Vec<String>>, token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>, secret_key: Option<String>) -> Self {
        Self::Oauth2Jwt { provider, algorithm, key_id, issuer, audience, subject, expiration_seconds, extra_params: Some(extra_params), scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_scopes(provider: Option<String>, algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: Option<String>, audience: Option<String>, subject: Option<String>, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, Option<String>>>, scopes: Vec<String>, token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>, secret_key: Option<String>) -> Self {
        Self::Oauth2Jwt { provider, algorithm, key_id, issuer, audience, subject, expiration_seconds, extra_params, scopes: Some(scopes), token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_token_response_field(provider: Option<String>, algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: Option<String>, audience: Option<String>, subject: Option<String>, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, Option<String>>>, scopes: Option<Vec<String>>, token_response_field: UpdateOAuth2JwtRequestTokenResponseField, secret_key: Option<String>) -> Self {
        Self::Oauth2Jwt { provider, algorithm, key_id, issuer, audience, subject, expiration_seconds, extra_params, scopes, token_response_field: Some(token_response_field), secret_key }
    }

    pub fn oauth2jwt_with_secret_key(provider: Option<String>, algorithm: Option<UpdateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: Option<String>, audience: Option<String>, subject: Option<String>, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, Option<String>>>, scopes: Option<Vec<String>>, token_response_field: Option<UpdateOAuth2JwtRequestTokenResponseField>, secret_key: String) -> Self {
        Self::Oauth2Jwt { provider, algorithm, key_id, issuer, audience, subject, expiration_seconds, extra_params, scopes, token_response_field, secret_key: Some(secret_key) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
