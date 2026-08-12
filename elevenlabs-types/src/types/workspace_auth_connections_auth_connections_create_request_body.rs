pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "auth_type")]
#[non_exhaustive]
pub enum AuthConnectionsCreateRequestBody {
        #[serde(rename = "oauth2_client_credentials")]
        #[non_exhaustive]
        Oauth2ClientCredentials {
            #[serde(default)]
            name: String,
            #[serde(default)]
            provider: String,
            #[serde(default)]
            client_id: String,
            #[serde(default)]
            token_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            scopes: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            extra_params: Option<HashMap<String, String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            basic_auth_in_header: Option<bool>,
            #[serde(default)]
            client_secret: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            custom_headers: Option<HashMap<String, String>>,
        },

        #[serde(rename = "custom_header_auth")]
        #[non_exhaustive]
        CustomHeaderAuth {
            #[serde(default)]
            name: String,
            #[serde(default)]
            provider: String,
            #[serde(default)]
            header_name: String,
            #[serde(default)]
            token: String,
        },

        #[serde(rename = "basic_auth")]
        #[non_exhaustive]
        BasicAuth {
            #[serde(default)]
            name: String,
            #[serde(default)]
            provider: String,
            #[serde(default)]
            username: String,
            #[serde(default)]
            password: String,
        },

        #[serde(rename = "bearer_auth")]
        #[non_exhaustive]
        BearerAuth {
            #[serde(default)]
            name: String,
            #[serde(default)]
            provider: String,
            #[serde(default)]
            token: String,
        },

        #[serde(rename = "oauth2_jwt")]
        #[non_exhaustive]
        Oauth2Jwt {
            #[serde(default)]
            name: String,
            #[serde(default)]
            provider: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            algorithm: Option<CreateOAuth2JwtRequestAlgorithm>,
            #[serde(skip_serializing_if = "Option::is_none")]
            key_id: Option<String>,
            #[serde(default)]
            issuer: String,
            #[serde(default)]
            audience: String,
            #[serde(default)]
            subject: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            expiration_seconds: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            extra_params: Option<HashMap<String, String>>,
            #[serde(default)]
            token_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            scopes: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            token_response_field: Option<CreateOAuth2JwtRequestTokenResponseField>,
            #[serde(default)]
            secret_key: String,
        },

        #[serde(rename = "private_key_jwt")]
        #[non_exhaustive]
        PrivateKeyJwt {
            #[serde(default)]
            name: String,
            #[serde(default)]
            provider: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            algorithm: Option<CreatePrivateKeyJwtRequestAlgorithm>,
            #[serde(skip_serializing_if = "Option::is_none")]
            key_id: Option<String>,
            #[serde(default)]
            issuer: String,
            #[serde(default)]
            audience: String,
            #[serde(default)]
            subject: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            expiration_seconds: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            extra_params: Option<HashMap<String, String>>,
            #[serde(default)]
            secret_key: String,
        },

        #[serde(rename = "mtls")]
        #[non_exhaustive]
        Mtls {
            #[serde(default)]
            name: String,
            #[serde(default)]
            provider: String,
            #[serde(default)]
            client_certificate: String,
            #[serde(default)]
            client_key: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            ca_certificate: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            key_passphrase: Option<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl AuthConnectionsCreateRequestBody {
    pub fn oauth2client_credentials(name: String, provider: String, client_id: String, token_url: String, client_secret: String) -> Self {
        Self::Oauth2ClientCredentials { name, provider, client_id, token_url, scopes: None, extra_params: None, basic_auth_in_header: None, client_secret, custom_headers: None }
    }

    pub fn custom_header_auth(name: String, provider: String, header_name: String, token: String) -> Self {
        Self::CustomHeaderAuth { name, provider, header_name, token }
    }

    pub fn basic_auth(name: String, provider: String, username: String, password: String) -> Self {
        Self::BasicAuth { name, provider, username, password }
    }

    pub fn bearer_auth(name: String, provider: String, token: String) -> Self {
        Self::BearerAuth { name, provider, token }
    }

    pub fn oauth2jwt(name: String, provider: String, issuer: String, audience: String, subject: String, token_url: String, secret_key: String) -> Self {
        Self::Oauth2Jwt { name, provider, algorithm: None, key_id: None, issuer, audience, subject, expiration_seconds: None, extra_params: None, token_url, scopes: None, token_response_field: None, secret_key }
    }

    pub fn private_key_jwt(name: String, provider: String, issuer: String, audience: String, subject: String, secret_key: String) -> Self {
        Self::PrivateKeyJwt { name, provider, algorithm: None, key_id: None, issuer, audience, subject, expiration_seconds: None, extra_params: None, secret_key }
    }

    pub fn mtls(name: String, provider: String, client_certificate: String, client_key: String) -> Self {
        Self::Mtls { name, provider, client_certificate, client_key, ca_certificate: None, key_passphrase: None }
    }

    pub fn oauth2client_credentials_with_scopes(name: String, provider: String, client_id: String, token_url: String, scopes: Vec<String>, extra_params: Option<HashMap<String, String>>, basic_auth_in_header: Option<bool>, client_secret: String, custom_headers: Option<HashMap<String, String>>) -> Self {
        Self::Oauth2ClientCredentials { name, provider, client_id, token_url, scopes: Some(scopes), extra_params, basic_auth_in_header, client_secret, custom_headers }
    }

    pub fn oauth2client_credentials_with_extra_params(name: String, provider: String, client_id: String, token_url: String, scopes: Option<Vec<String>>, extra_params: HashMap<String, String>, basic_auth_in_header: Option<bool>, client_secret: String, custom_headers: Option<HashMap<String, String>>) -> Self {
        Self::Oauth2ClientCredentials { name, provider, client_id, token_url, scopes, extra_params: Some(extra_params), basic_auth_in_header, client_secret, custom_headers }
    }

    pub fn oauth2client_credentials_with_basic_auth_in_header(name: String, provider: String, client_id: String, token_url: String, scopes: Option<Vec<String>>, extra_params: Option<HashMap<String, String>>, basic_auth_in_header: bool, client_secret: String, custom_headers: Option<HashMap<String, String>>) -> Self {
        Self::Oauth2ClientCredentials { name, provider, client_id, token_url, scopes, extra_params, basic_auth_in_header: Some(basic_auth_in_header), client_secret, custom_headers }
    }

    pub fn oauth2client_credentials_with_custom_headers(name: String, provider: String, client_id: String, token_url: String, scopes: Option<Vec<String>>, extra_params: Option<HashMap<String, String>>, basic_auth_in_header: Option<bool>, client_secret: String, custom_headers: HashMap<String, String>) -> Self {
        Self::Oauth2ClientCredentials { name, provider, client_id, token_url, scopes, extra_params, basic_auth_in_header, client_secret, custom_headers: Some(custom_headers) }
    }

    pub fn oauth2jwt_with_algorithm(name: String, provider: String, algorithm: CreateOAuth2JwtRequestAlgorithm, key_id: Option<String>, issuer: String, audience: String, subject: String, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, String>>, token_url: String, scopes: Option<Vec<String>>, token_response_field: Option<CreateOAuth2JwtRequestTokenResponseField>, secret_key: String) -> Self {
        Self::Oauth2Jwt { name, provider, algorithm: Some(algorithm), key_id, issuer, audience, subject, expiration_seconds, extra_params, token_url, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_key_id(name: String, provider: String, algorithm: Option<CreateOAuth2JwtRequestAlgorithm>, key_id: String, issuer: String, audience: String, subject: String, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, String>>, token_url: String, scopes: Option<Vec<String>>, token_response_field: Option<CreateOAuth2JwtRequestTokenResponseField>, secret_key: String) -> Self {
        Self::Oauth2Jwt { name, provider, algorithm, key_id: Some(key_id), issuer, audience, subject, expiration_seconds, extra_params, token_url, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_expiration_seconds(name: String, provider: String, algorithm: Option<CreateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: String, audience: String, subject: String, expiration_seconds: i64, extra_params: Option<HashMap<String, String>>, token_url: String, scopes: Option<Vec<String>>, token_response_field: Option<CreateOAuth2JwtRequestTokenResponseField>, secret_key: String) -> Self {
        Self::Oauth2Jwt { name, provider, algorithm, key_id, issuer, audience, subject, expiration_seconds: Some(expiration_seconds), extra_params, token_url, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_extra_params(name: String, provider: String, algorithm: Option<CreateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: String, audience: String, subject: String, expiration_seconds: Option<i64>, extra_params: HashMap<String, String>, token_url: String, scopes: Option<Vec<String>>, token_response_field: Option<CreateOAuth2JwtRequestTokenResponseField>, secret_key: String) -> Self {
        Self::Oauth2Jwt { name, provider, algorithm, key_id, issuer, audience, subject, expiration_seconds, extra_params: Some(extra_params), token_url, scopes, token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_scopes(name: String, provider: String, algorithm: Option<CreateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: String, audience: String, subject: String, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, String>>, token_url: String, scopes: Vec<String>, token_response_field: Option<CreateOAuth2JwtRequestTokenResponseField>, secret_key: String) -> Self {
        Self::Oauth2Jwt { name, provider, algorithm, key_id, issuer, audience, subject, expiration_seconds, extra_params, token_url, scopes: Some(scopes), token_response_field, secret_key }
    }

    pub fn oauth2jwt_with_token_response_field(name: String, provider: String, algorithm: Option<CreateOAuth2JwtRequestAlgorithm>, key_id: Option<String>, issuer: String, audience: String, subject: String, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, String>>, token_url: String, scopes: Option<Vec<String>>, token_response_field: CreateOAuth2JwtRequestTokenResponseField, secret_key: String) -> Self {
        Self::Oauth2Jwt { name, provider, algorithm, key_id, issuer, audience, subject, expiration_seconds, extra_params, token_url, scopes, token_response_field: Some(token_response_field), secret_key }
    }

    pub fn private_key_jwt_with_algorithm(name: String, provider: String, algorithm: CreatePrivateKeyJwtRequestAlgorithm, key_id: Option<String>, issuer: String, audience: String, subject: String, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, String>>, secret_key: String) -> Self {
        Self::PrivateKeyJwt { name, provider, algorithm: Some(algorithm), key_id, issuer, audience, subject, expiration_seconds, extra_params, secret_key }
    }

    pub fn private_key_jwt_with_key_id(name: String, provider: String, algorithm: Option<CreatePrivateKeyJwtRequestAlgorithm>, key_id: String, issuer: String, audience: String, subject: String, expiration_seconds: Option<i64>, extra_params: Option<HashMap<String, String>>, secret_key: String) -> Self {
        Self::PrivateKeyJwt { name, provider, algorithm, key_id: Some(key_id), issuer, audience, subject, expiration_seconds, extra_params, secret_key }
    }

    pub fn private_key_jwt_with_expiration_seconds(name: String, provider: String, algorithm: Option<CreatePrivateKeyJwtRequestAlgorithm>, key_id: Option<String>, issuer: String, audience: String, subject: String, expiration_seconds: i64, extra_params: Option<HashMap<String, String>>, secret_key: String) -> Self {
        Self::PrivateKeyJwt { name, provider, algorithm, key_id, issuer, audience, subject, expiration_seconds: Some(expiration_seconds), extra_params, secret_key }
    }

    pub fn private_key_jwt_with_extra_params(name: String, provider: String, algorithm: Option<CreatePrivateKeyJwtRequestAlgorithm>, key_id: Option<String>, issuer: String, audience: String, subject: String, expiration_seconds: Option<i64>, extra_params: HashMap<String, String>, secret_key: String) -> Self {
        Self::PrivateKeyJwt { name, provider, algorithm, key_id, issuer, audience, subject, expiration_seconds, extra_params: Some(extra_params), secret_key }
    }

    pub fn mtls_with_ca_certificate(name: String, provider: String, client_certificate: String, client_key: String, ca_certificate: String, key_passphrase: Option<String>) -> Self {
        Self::Mtls { name, provider, client_certificate, client_key, ca_certificate: Some(ca_certificate), key_passphrase }
    }

    pub fn mtls_with_key_passphrase(name: String, provider: String, client_certificate: String, client_key: String, ca_certificate: Option<String>, key_passphrase: String) -> Self {
        Self::Mtls { name, provider, client_certificate, client_key, ca_certificate, key_passphrase: Some(key_passphrase) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
