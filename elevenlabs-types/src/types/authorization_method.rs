pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuthorizationMethod {
    Invalid,
    Public,
    AuthorizationHeader,
    SignedUrl,
    ShareableLink,
    LivekitToken,
    LivekitTokenWebsite,
    GenesysApiKey,
    AvayaApiKey,
    AudiocodesApiKey,
    Whatsapp,
    Sms,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AuthorizationMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Invalid => serializer.serialize_str("invalid"),
            Self::Public => serializer.serialize_str("public"),
            Self::AuthorizationHeader => serializer.serialize_str("authorization_header"),
            Self::SignedUrl => serializer.serialize_str("signed_url"),
            Self::ShareableLink => serializer.serialize_str("shareable_link"),
            Self::LivekitToken => serializer.serialize_str("livekit_token"),
            Self::LivekitTokenWebsite => serializer.serialize_str("livekit_token_website"),
            Self::GenesysApiKey => serializer.serialize_str("genesys_api_key"),
            Self::AvayaApiKey => serializer.serialize_str("avaya_api_key"),
            Self::AudiocodesApiKey => serializer.serialize_str("audiocodes_api_key"),
            Self::Whatsapp => serializer.serialize_str("whatsapp"),
            Self::Sms => serializer.serialize_str("sms"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AuthorizationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "invalid" => Ok(Self::Invalid),
            "public" => Ok(Self::Public),
            "authorization_header" => Ok(Self::AuthorizationHeader),
            "signed_url" => Ok(Self::SignedUrl),
            "shareable_link" => Ok(Self::ShareableLink),
            "livekit_token" => Ok(Self::LivekitToken),
            "livekit_token_website" => Ok(Self::LivekitTokenWebsite),
            "genesys_api_key" => Ok(Self::GenesysApiKey),
            "avaya_api_key" => Ok(Self::AvayaApiKey),
            "audiocodes_api_key" => Ok(Self::AudiocodesApiKey),
            "whatsapp" => Ok(Self::Whatsapp),
            "sms" => Ok(Self::Sms),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AuthorizationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid => write!(f, "invalid"),
            Self::Public => write!(f, "public"),
            Self::AuthorizationHeader => write!(f, "authorization_header"),
            Self::SignedUrl => write!(f, "signed_url"),
            Self::ShareableLink => write!(f, "shareable_link"),
            Self::LivekitToken => write!(f, "livekit_token"),
            Self::LivekitTokenWebsite => write!(f, "livekit_token_website"),
            Self::GenesysApiKey => write!(f, "genesys_api_key"),
            Self::AvayaApiKey => write!(f, "avaya_api_key"),
            Self::AudiocodesApiKey => write!(f, "audiocodes_api_key"),
            Self::Whatsapp => write!(f, "whatsapp"),
            Self::Sms => write!(f, "sms"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
