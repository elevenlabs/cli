pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPost {
    /// List of pronunciation rules. Rule can be either:
    /// an alias rule: {'string_to_replace': 'a', 'type': 'alias', 'alias': 'b', }
    /// or a phoneme rule: {'string_to_replace': 'a', 'type': 'phoneme', 'phoneme': 'b', 'alphabet': 'ipa' }
    #[serde(default)]
    pub rules: Vec<BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostRulesItem>,
    /// The name of the pronunciation dictionary, used for identification only.
    #[serde(default)]
    pub name: String,
    /// A description of the pronunciation dictionary, used for identification only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Should be one of 'admin', 'editor' or 'viewer'. If not provided, defaults to no access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_access: Option<BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostWorkspaceAccess>,
}

impl BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPost {
    pub fn builder() -> BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostBuilder {
        <BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostBuilder {
    rules: Option<Vec<BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostRulesItem>>,
    name: Option<String>,
    description: Option<String>,
    workspace_access: Option<BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostWorkspaceAccess>,
}

impl BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostBuilder {
    pub fn rules(mut self, value: Vec<BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostRulesItem>) -> Self {
        self.rules = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn workspace_access(mut self, value: BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostWorkspaceAccess) -> Self {
        self.workspace_access = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rules`](BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostBuilder::rules)
    /// - [`name`](BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPostBuilder::name)
    pub fn build(self) -> Result<BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPost, BuildError> {
        Ok(BodyAddAPronunciationDictionaryV1PronunciationDictionariesAddFromRulesPost {
            rules: self.rules.ok_or_else(|| BuildError::missing_field("rules"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            workspace_access: self.workspace_access,
        })
    }
}

