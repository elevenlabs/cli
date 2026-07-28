//! API client and types for the ElevenLabs API Documentation
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints

pub mod resources;

pub use resources::{
    AgentsClient, ApiClient, AudioIsolationClient, AudioNativeClient, DubbingClient,
    EnvironmentVariablesClient, ForcedAlignmentClient, HistoryClient, ModelsClient, MusicClient,
    ProductionsClient, PronunciationDictionariesClient, SamplesClient, ServiceAccountsClient,
    SpeechEngineClient, SpeechToSpeechClient, SpeechToTextClient, StudioClient,
    TextToDialogueClient, TextToSoundEffectsClient, TextToSpeechClient, TextToVoiceClient,
    TokensClient, UsageClient, UserClient, VoicesClient, WebhooksClient, WorkspaceClient,
    WorkspacesClient,
};

pub use elevenlabs_types::*;
