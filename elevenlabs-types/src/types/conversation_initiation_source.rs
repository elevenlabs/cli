pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Enum representing the possible sources for conversation initiation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationInitiationSource {
    Unknown,
    AndroidSdk,
    NodeJsSdk,
    ReactNativeSdk,
    ReactSdk,
    JsSdk,
    PythonSdk,
    Widget,
    SipTrunk,
    Twilio,
    Exotel,
    Genesys,
    Avaya,
    Audiocodes,
    SwiftSdk,
    Whatsapp,
    TwilioSms,
    FlutterSdk,
    ZendeskIntegration,
    SlackIntegration,
    TelegramIntegration,
    IntercomIntegration,
    FreshdeskIntegration,
    SalesforceIntegration,
    TemplatePreview,
    GenesysBotConnector,
    SubagentTool,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationInitiationSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::AndroidSdk => serializer.serialize_str("android_sdk"),
            Self::NodeJsSdk => serializer.serialize_str("node_js_sdk"),
            Self::ReactNativeSdk => serializer.serialize_str("react_native_sdk"),
            Self::ReactSdk => serializer.serialize_str("react_sdk"),
            Self::JsSdk => serializer.serialize_str("js_sdk"),
            Self::PythonSdk => serializer.serialize_str("python_sdk"),
            Self::Widget => serializer.serialize_str("widget"),
            Self::SipTrunk => serializer.serialize_str("sip_trunk"),
            Self::Twilio => serializer.serialize_str("twilio"),
            Self::Exotel => serializer.serialize_str("exotel"),
            Self::Genesys => serializer.serialize_str("genesys"),
            Self::Avaya => serializer.serialize_str("avaya"),
            Self::Audiocodes => serializer.serialize_str("audiocodes"),
            Self::SwiftSdk => serializer.serialize_str("swift_sdk"),
            Self::Whatsapp => serializer.serialize_str("whatsapp"),
            Self::TwilioSms => serializer.serialize_str("twilio_sms"),
            Self::FlutterSdk => serializer.serialize_str("flutter_sdk"),
            Self::ZendeskIntegration => serializer.serialize_str("zendesk_integration"),
            Self::SlackIntegration => serializer.serialize_str("slack_integration"),
            Self::TelegramIntegration => serializer.serialize_str("telegram_integration"),
            Self::IntercomIntegration => serializer.serialize_str("intercom_integration"),
            Self::FreshdeskIntegration => serializer.serialize_str("freshdesk_integration"),
            Self::SalesforceIntegration => serializer.serialize_str("salesforce_integration"),
            Self::TemplatePreview => serializer.serialize_str("template_preview"),
            Self::GenesysBotConnector => serializer.serialize_str("genesys_bot_connector"),
            Self::SubagentTool => serializer.serialize_str("subagent_tool"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationInitiationSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unknown" => Ok(Self::Unknown),
            "android_sdk" => Ok(Self::AndroidSdk),
            "node_js_sdk" => Ok(Self::NodeJsSdk),
            "react_native_sdk" => Ok(Self::ReactNativeSdk),
            "react_sdk" => Ok(Self::ReactSdk),
            "js_sdk" => Ok(Self::JsSdk),
            "python_sdk" => Ok(Self::PythonSdk),
            "widget" => Ok(Self::Widget),
            "sip_trunk" => Ok(Self::SipTrunk),
            "twilio" => Ok(Self::Twilio),
            "exotel" => Ok(Self::Exotel),
            "genesys" => Ok(Self::Genesys),
            "avaya" => Ok(Self::Avaya),
            "audiocodes" => Ok(Self::Audiocodes),
            "swift_sdk" => Ok(Self::SwiftSdk),
            "whatsapp" => Ok(Self::Whatsapp),
            "twilio_sms" => Ok(Self::TwilioSms),
            "flutter_sdk" => Ok(Self::FlutterSdk),
            "zendesk_integration" => Ok(Self::ZendeskIntegration),
            "slack_integration" => Ok(Self::SlackIntegration),
            "telegram_integration" => Ok(Self::TelegramIntegration),
            "intercom_integration" => Ok(Self::IntercomIntegration),
            "freshdesk_integration" => Ok(Self::FreshdeskIntegration),
            "salesforce_integration" => Ok(Self::SalesforceIntegration),
            "template_preview" => Ok(Self::TemplatePreview),
            "genesys_bot_connector" => Ok(Self::GenesysBotConnector),
            "subagent_tool" => Ok(Self::SubagentTool),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationInitiationSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::AndroidSdk => write!(f, "android_sdk"),
            Self::NodeJsSdk => write!(f, "node_js_sdk"),
            Self::ReactNativeSdk => write!(f, "react_native_sdk"),
            Self::ReactSdk => write!(f, "react_sdk"),
            Self::JsSdk => write!(f, "js_sdk"),
            Self::PythonSdk => write!(f, "python_sdk"),
            Self::Widget => write!(f, "widget"),
            Self::SipTrunk => write!(f, "sip_trunk"),
            Self::Twilio => write!(f, "twilio"),
            Self::Exotel => write!(f, "exotel"),
            Self::Genesys => write!(f, "genesys"),
            Self::Avaya => write!(f, "avaya"),
            Self::Audiocodes => write!(f, "audiocodes"),
            Self::SwiftSdk => write!(f, "swift_sdk"),
            Self::Whatsapp => write!(f, "whatsapp"),
            Self::TwilioSms => write!(f, "twilio_sms"),
            Self::FlutterSdk => write!(f, "flutter_sdk"),
            Self::ZendeskIntegration => write!(f, "zendesk_integration"),
            Self::SlackIntegration => write!(f, "slack_integration"),
            Self::TelegramIntegration => write!(f, "telegram_integration"),
            Self::IntercomIntegration => write!(f, "intercom_integration"),
            Self::FreshdeskIntegration => write!(f, "freshdesk_integration"),
            Self::SalesforceIntegration => write!(f, "salesforce_integration"),
            Self::TemplatePreview => write!(f, "template_preview"),
            Self::GenesysBotConnector => write!(f, "genesys_bot_connector"),
            Self::SubagentTool => write!(f, "subagent_tool"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
