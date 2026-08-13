//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **History**
//! - **TextToSoundEffects**
//! - **AudioIsolation**
//! - **Samples**
//! - **TextToSpeech**
//! - **TextToDialogue**
//! - **SpeechToSpeech**
//! - **TextToVoice**
//! - **User**
//! - **Voices**
//! - **Studio**
//! - **dubbing**
//! - **Models**
//! - **AudioNative**
//! - **Usage**
//! - **PronunciationDictionaries**
//! - **Workspace**
//! - **ServiceAccounts**
//! - **Webhooks**
//! - **Music**
//! - **SpeechToText**
//! - **ForcedAlignment**
//! - **Agents**
//! - **Speech Engine**
//! - **EnvironmentVariables**
//! - **Assets**
//! - **Flows**
//! - **Productions**
//! - **Tokens**
//! - **Workspaces**

use crate::{ApiError, ClientConfig};

pub mod agents;
pub mod assets;
pub mod audio_isolation;
pub mod audio_native;
pub mod dubbing;
pub mod environment_variables;
pub mod flows;
pub mod forced_alignment;
pub mod history;
pub mod models;
pub mod music;
pub mod productions;
pub mod pronunciation_dictionaries;
pub mod samples;
pub mod service_accounts;
pub mod speech_engine;
pub mod speech_to_speech;
pub mod speech_to_text;
pub mod studio;
pub mod text_to_dialogue;
pub mod text_to_sound_effects;
pub mod text_to_speech;
pub mod text_to_voice;
pub mod tokens;
pub mod usage;
pub mod user;
pub mod voices;
pub mod webhooks;
pub mod workspace;
pub mod workspaces;
pub struct ApiClient {
    pub config: ClientConfig,
    pub history: HistoryClient,
    pub text_to_sound_effects: TextToSoundEffectsClient,
    pub audio_isolation: AudioIsolationClient,
    pub samples: SamplesClient,
    pub text_to_speech: TextToSpeechClient,
    pub text_to_dialogue: TextToDialogueClient,
    pub speech_to_speech: SpeechToSpeechClient,
    pub text_to_voice: TextToVoiceClient,
    pub user: UserClient,
    pub voices: VoicesClient,
    pub studio: StudioClient,
    pub dubbing: DubbingClient,
    pub models: ModelsClient,
    pub audio_native: AudioNativeClient,
    pub usage: UsageClient,
    pub pronunciation_dictionaries: PronunciationDictionariesClient,
    pub workspace: WorkspaceClient,
    pub service_accounts: ServiceAccountsClient,
    pub webhooks: WebhooksClient,
    pub music: MusicClient,
    pub speech_to_text: SpeechToTextClient,
    pub forced_alignment: ForcedAlignmentClient,
    pub agents: AgentsClient,
    pub speech_engine: SpeechEngineClient,
    pub environment_variables: EnvironmentVariablesClient,
    pub assets: AssetsClient,
    pub flows: FlowsClient,
    pub productions: ProductionsClient,
    pub tokens: TokensClient,
    pub workspaces: WorkspacesClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            history: HistoryClient::new(config.clone())?,
            text_to_sound_effects: TextToSoundEffectsClient::new(config.clone())?,
            audio_isolation: AudioIsolationClient::new(config.clone())?,
            samples: SamplesClient::new(config.clone())?,
            text_to_speech: TextToSpeechClient::new(config.clone())?,
            text_to_dialogue: TextToDialogueClient::new(config.clone())?,
            speech_to_speech: SpeechToSpeechClient::new(config.clone())?,
            text_to_voice: TextToVoiceClient::new(config.clone())?,
            user: UserClient::new(config.clone())?,
            voices: VoicesClient::new(config.clone())?,
            studio: StudioClient::new(config.clone())?,
            dubbing: DubbingClient::new(config.clone())?,
            models: ModelsClient::new(config.clone())?,
            audio_native: AudioNativeClient::new(config.clone())?,
            usage: UsageClient::new(config.clone())?,
            pronunciation_dictionaries: PronunciationDictionariesClient::new(config.clone())?,
            workspace: WorkspaceClient::new(config.clone())?,
            service_accounts: ServiceAccountsClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
            music: MusicClient::new(config.clone())?,
            speech_to_text: SpeechToTextClient::new(config.clone())?,
            forced_alignment: ForcedAlignmentClient::new(config.clone())?,
            agents: AgentsClient::new(config.clone())?,
            speech_engine: SpeechEngineClient::new(config.clone())?,
            environment_variables: EnvironmentVariablesClient::new(config.clone())?,
            assets: AssetsClient::new(config.clone())?,
            flows: FlowsClient::new(config.clone())?,
            productions: ProductionsClient::new(config.clone())?,
            tokens: TokensClient::new(config.clone())?,
            workspaces: WorkspacesClient::new(config.clone())?,
        })
    }
}

pub use agents::AgentsClient;
pub use assets::AssetsClient;
pub use audio_isolation::AudioIsolationClient;
pub use audio_native::AudioNativeClient;
pub use dubbing::DubbingClient;
pub use environment_variables::EnvironmentVariablesClient;
pub use flows::FlowsClient;
pub use forced_alignment::ForcedAlignmentClient;
pub use history::HistoryClient;
pub use models::ModelsClient;
pub use music::MusicClient;
pub use productions::ProductionsClient;
pub use pronunciation_dictionaries::PronunciationDictionariesClient;
pub use samples::SamplesClient;
pub use service_accounts::ServiceAccountsClient;
pub use speech_engine::SpeechEngineClient;
pub use speech_to_speech::SpeechToSpeechClient;
pub use speech_to_text::SpeechToTextClient;
pub use studio::StudioClient;
pub use text_to_dialogue::TextToDialogueClient;
pub use text_to_sound_effects::TextToSoundEffectsClient;
pub use text_to_speech::TextToSpeechClient;
pub use text_to_voice::TextToVoiceClient;
pub use tokens::TokensClient;
pub use usage::UsageClient;
pub use user::UserClient;
pub use voices::VoicesClient;
pub use webhooks::WebhooksClient;
pub use workspace::WorkspaceClient;
pub use workspaces::WorkspacesClient;
