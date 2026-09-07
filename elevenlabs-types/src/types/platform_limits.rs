pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Unified container for all platform limits.
/// 
/// Can be used by:
/// - Billing groups (WorkspaceGroupDBModel.platform_limits)
/// - Child workspaces (SubscriptionDBModel.platform_limits)
/// - API keys (XiApiKeyMetadataDBModel.platform_limits)
/// 
/// All fields are required when platform_limits exists. Use limit=None for unlimited.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlatformLimits {
    /// Credit usage limit (limit=None means unlimited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credits: Option<StoredUsagePlatformLimit>,
    /// Professional Voice Clone count limit (limit=None means unlimited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pvc: Option<ComputedUsagePlatformLimit>,
    /// TTS concurrency limit (limit=None means unlimited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<ComputedUsagePlatformLimit>,
    /// Dubbing concurrency limit (limit=None means unlimited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dubbing_concurrency: Option<ComputedUsagePlatformLimit>,
    /// Music generation concurrency limit (limit=None means unlimited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_concurrency: Option<ComputedUsagePlatformLimit>,
}

impl PlatformLimits {
    pub fn builder() -> PlatformLimitsBuilder {
        <PlatformLimitsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlatformLimitsBuilder {
    credits: Option<StoredUsagePlatformLimit>,
    pvc: Option<ComputedUsagePlatformLimit>,
    concurrency: Option<ComputedUsagePlatformLimit>,
    dubbing_concurrency: Option<ComputedUsagePlatformLimit>,
    music_concurrency: Option<ComputedUsagePlatformLimit>,
}

impl PlatformLimitsBuilder {
    pub fn credits(mut self, value: StoredUsagePlatformLimit) -> Self {
        self.credits = Some(value);
        self
    }

    pub fn pvc(mut self, value: ComputedUsagePlatformLimit) -> Self {
        self.pvc = Some(value);
        self
    }

    pub fn concurrency(mut self, value: ComputedUsagePlatformLimit) -> Self {
        self.concurrency = Some(value);
        self
    }

    pub fn dubbing_concurrency(mut self, value: ComputedUsagePlatformLimit) -> Self {
        self.dubbing_concurrency = Some(value);
        self
    }

    pub fn music_concurrency(mut self, value: ComputedUsagePlatformLimit) -> Self {
        self.music_concurrency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlatformLimits`].
    pub fn build(self) -> Result<PlatformLimits, BuildError> {
        Ok(PlatformLimits {
            credits: self.credits,
            pvc: self.pvc,
            concurrency: self.concurrency,
            dubbing_concurrency: self.dubbing_concurrency,
            music_concurrency: self.music_concurrency,
        })
    }
}
