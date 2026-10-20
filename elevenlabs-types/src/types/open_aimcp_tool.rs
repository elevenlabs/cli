pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenAimcpTool {
    #[serde(default)]
    pub server_label: String,
    #[serde(default)]
    pub server_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval: Option<OpenAimcpToolRequireApproval>,
}

impl OpenAimcpTool {
    pub fn builder() -> OpenAimcpToolBuilder {
        <OpenAimcpToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAimcpToolBuilder {
    server_label: Option<String>,
    server_url: Option<String>,
    authorization: Option<String>,
    allowed_tools: Option<Vec<String>>,
    require_approval: Option<OpenAimcpToolRequireApproval>,
}

impl OpenAimcpToolBuilder {
    pub fn server_label(mut self, value: impl Into<String>) -> Self {
        self.server_label = Some(value.into());
        self
    }

    pub fn server_url(mut self, value: impl Into<String>) -> Self {
        self.server_url = Some(value.into());
        self
    }

    pub fn authorization(mut self, value: impl Into<String>) -> Self {
        self.authorization = Some(value.into());
        self
    }

    pub fn allowed_tools(mut self, value: Vec<String>) -> Self {
        self.allowed_tools = Some(value);
        self
    }

    pub fn require_approval(mut self, value: OpenAimcpToolRequireApproval) -> Self {
        self.require_approval = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAimcpTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`server_label`](OpenAimcpToolBuilder::server_label)
    /// - [`server_url`](OpenAimcpToolBuilder::server_url)
    pub fn build(self) -> Result<OpenAimcpTool, BuildError> {
        Ok(OpenAimcpTool {
            server_label: self.server_label.ok_or_else(|| BuildError::missing_field("server_label"))?,
            server_url: self.server_url.ok_or_else(|| BuildError::missing_field("server_url"))?,
            authorization: self.authorization,
            allowed_tools: self.allowed_tools,
            require_approval: self.require_approval,
        })
    }
}
