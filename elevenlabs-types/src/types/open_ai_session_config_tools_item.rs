pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum OpenAiSessionConfigToolsItem {
        #[serde(rename = "function")]
        #[non_exhaustive]
        Function {
            #[serde(default)]
            name: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            parameters: Option<HashMap<String, serde_json::Value>>,
        },

        #[serde(rename = "mcp")]
        #[non_exhaustive]
        Mcp {
            #[serde(default)]
            server_label: String,
            #[serde(default)]
            server_url: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            authorization: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allowed_tools: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            require_approval: Option<OpenAimcpToolRequireApproval>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl OpenAiSessionConfigToolsItem {
    pub fn function(name: String) -> Self {
        Self::Function { name, description: None, parameters: None }
    }

    pub fn mcp(server_label: String, server_url: String) -> Self {
        Self::Mcp { server_label, server_url, authorization: None, allowed_tools: None, require_approval: None }
    }

    pub fn function_with_description(name: String, description: String, parameters: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self::Function { name, description: Some(description), parameters }
    }

    pub fn function_with_parameters(name: String, description: Option<String>, parameters: HashMap<String, serde_json::Value>) -> Self {
        Self::Function { name, description, parameters: Some(parameters) }
    }

    pub fn mcp_with_authorization(server_label: String, server_url: String, authorization: String, allowed_tools: Option<Vec<String>>, require_approval: Option<OpenAimcpToolRequireApproval>) -> Self {
        Self::Mcp { server_label, server_url, authorization: Some(authorization), allowed_tools, require_approval }
    }

    pub fn mcp_with_allowed_tools(server_label: String, server_url: String, authorization: Option<String>, allowed_tools: Vec<String>, require_approval: Option<OpenAimcpToolRequireApproval>) -> Self {
        Self::Mcp { server_label, server_url, authorization, allowed_tools: Some(allowed_tools), require_approval }
    }

    pub fn mcp_with_require_approval(server_label: String, server_url: String, authorization: Option<String>, allowed_tools: Option<Vec<String>>, require_approval: OpenAimcpToolRequireApproval) -> Self {
        Self::Mcp { server_label, server_url, authorization, allowed_tools, require_approval: Some(require_approval) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
