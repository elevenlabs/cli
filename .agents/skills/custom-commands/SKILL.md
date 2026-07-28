---
name: elevenlabs-custom-commands
description: How to author custom commands for the elevenlabs CLI using the co-generated SDK.
---

# Custom Commands for `elevenlabs`

## Overview

The `elevenlabs` CLI supports user-authored custom commands that are
compiled into the binary alongside the auto-generated API commands.
Custom commands get a fully-wired SDK client that inherits the CLI's
auth, retries, TLS, base URL, and global headers — zero configuration required.

## Architecture

```
cli/elevenlabs/custom.rs    ← Your command handlers (protected by .fernignore)
cli/elevenlabs/sdk.rs       ← Generated bridge: client() + block_on()
cli/elevenlabs/main.rs      ← Generated entrypoint (calls custom::register)
elevenlabs-sdk/             ← Co-generated typed SDK crate
elevenlabs-types/           ← Co-generated typed model crate
```

## Adding a Custom Command

### 1. Edit `cli/elevenlabs/custom.rs`

This file is protected by `.fernignore` — `fern generate` will never
overwrite it. Register commands in the `register()` function:

```rust
use elevenlabs_sdk::api::*;

pub fn register(app: CliApp) -> CliApp {
    let app = app.command(
        clap::Command::new("get")
            .about("Get history item")
            .arg(clap::Arg::new("history_item_id").required(true))
        ,
        |matches, ctx| {
            let history_item_id = matches.get_one::<String>("history_item_id").unwrap();
            let client = super::sdk::client(ctx);
            let result = super::sdk::block_on(
                client.history.get(history_item_id),
            )?;
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            Ok(())
        },
    );
    app
}
```

Then build and test:
```bash
cargo build
elevenlabs get <history_item_id>
```

### 2. Available SDK Clients

The `super::sdk::client(ctx)` call returns a `elevenlabs_sdk::api::Client`
with the following sub-clients:

| Field | Type | Description |
|-------|------|-------------|
| `client.history` | `elevenlabs_sdk::api::HistoryClient` | history operations |
| `client.text_to_sound_effects` | `elevenlabs_sdk::api::TextToSoundEffectsClient` | text_to_sound_effects operations |
| `client.audio_isolation` | `elevenlabs_sdk::api::AudioIsolationClient` | audio_isolation operations |
| `client.samples` | `elevenlabs_sdk::api::SamplesClient` | samples operations |
| `client.text_to_speech` | `elevenlabs_sdk::api::TextToSpeechClient` | text_to_speech operations |
| `client.text_to_dialogue` | `elevenlabs_sdk::api::TextToDialogueClient` | text_to_dialogue operations |
| `client.speech_to_speech` | `elevenlabs_sdk::api::SpeechToSpeechClient` | speech_to_speech operations |
| `client.text_to_voice` | `elevenlabs_sdk::api::TextToVoiceClient` | text_to_voice operations |
| `client.preview` | `elevenlabs_sdk::api::PreviewClient` | preview operations |
| `client.user` | `elevenlabs_sdk::api::UserClient` | user operations |
| `client.subscription` | `elevenlabs_sdk::api::SubscriptionClient` | subscription operations |
| `client.voices` | `elevenlabs_sdk::api::VoicesClient` | voices operations |
| `client.settings` | `elevenlabs_sdk::api::SettingsClient3` | settings operations |
| `client.ivc` | `elevenlabs_sdk::api::IvcClient` | ivc operations |
| `client.pvc` | `elevenlabs_sdk::api::PvcClient` | pvc operations |
| `client.samples` | `elevenlabs_sdk::api::SamplesClient2` | samples operations |
| `client.audio` | `elevenlabs_sdk::api::AudioClient3` | audio operations |
| `client.waveform` | `elevenlabs_sdk::api::WaveformClient` | waveform operations |
| `client.speakers` | `elevenlabs_sdk::api::SpeakersClient` | speakers operations |
| `client.audio` | `elevenlabs_sdk::api::AudioClient4` | audio operations |
| `client.verification` | `elevenlabs_sdk::api::VerificationClient` | verification operations |
| `client.captcha` | `elevenlabs_sdk::api::CaptchaClient` | captcha operations |
| `client.samples` | `elevenlabs_sdk::api::SamplesClient3` | samples operations |
| `client.audio` | `elevenlabs_sdk::api::AudioClient5` | audio operations |
| `client.studio` | `elevenlabs_sdk::api::StudioClient` | studio operations |
| `client.projects` | `elevenlabs_sdk::api::ProjectsClient` | projects operations |
| `client.pronunciation_dictionaries` | `elevenlabs_sdk::api::PronunciationDictionariesClient2` | pronunciation_dictionaries operations |
| `client.content` | `elevenlabs_sdk::api::ContentClient` | content operations |
| `client.snapshots` | `elevenlabs_sdk::api::SnapshotsClient` | snapshots operations |
| `client.chapters` | `elevenlabs_sdk::api::ChaptersClient` | chapters operations |
| `client.snapshots` | `elevenlabs_sdk::api::SnapshotsClient2` | snapshots operations |
| `client.dubbing` | `elevenlabs_sdk::api::DubbingClient` | dubbing operations |
| `client.project` | `elevenlabs_sdk::api::ProjectClient` | project operations |
| `client.language` | `elevenlabs_sdk::api::LanguageClient` | language operations |
| `client.transcript` | `elevenlabs_sdk::api::TranscriptClient3` | transcript operations |
| `client.transcript` | `elevenlabs_sdk::api::TranscriptClient2` | transcript operations |
| `client.resource` | `elevenlabs_sdk::api::ResourceClient` | resource operations |
| `client.language` | `elevenlabs_sdk::api::LanguageClient2` | language operations |
| `client.segment` | `elevenlabs_sdk::api::SegmentClient` | segment operations |
| `client.speaker` | `elevenlabs_sdk::api::SpeakerClient` | speaker operations |
| `client.segment` | `elevenlabs_sdk::api::SegmentClient2` | segment operations |
| `client.audio` | `elevenlabs_sdk::api::AudioClient2` | audio operations |
| `client.transcript` | `elevenlabs_sdk::api::TranscriptClient` | transcript operations |
| `client.transcripts` | `elevenlabs_sdk::api::TranscriptsClient` | transcripts operations |
| `client.models` | `elevenlabs_sdk::api::ModelsClient` | models operations |
| `client.audio_native` | `elevenlabs_sdk::api::AudioNativeClient` | audio_native operations |
| `client.usage` | `elevenlabs_sdk::api::UsageClient` | usage operations |
| `client.pronunciation_dictionaries` | `elevenlabs_sdk::api::PronunciationDictionariesClient` | pronunciation_dictionaries operations |
| `client.rules` | `elevenlabs_sdk::api::RulesClient` | rules operations |
| `client.workspace` | `elevenlabs_sdk::api::WorkspaceClient` | workspace operations |
| `client.audit_logs` | `elevenlabs_sdk::api::AuditLogsClient` | audit_logs operations |
| `client.auth_connections` | `elevenlabs_sdk::api::AuthConnectionsClient` | auth_connections operations |
| `client.groups` | `elevenlabs_sdk::api::GroupsClient` | groups operations |
| `client.members` | `elevenlabs_sdk::api::MembersClient2` | members operations |
| `client.invites` | `elevenlabs_sdk::api::InvitesClient` | invites operations |
| `client.members` | `elevenlabs_sdk::api::MembersClient` | members operations |
| `client.resources` | `elevenlabs_sdk::api::ResourcesClient` | resources operations |
| `client.usage` | `elevenlabs_sdk::api::UsageClient2` | usage operations |
| `client.analytics` | `elevenlabs_sdk::api::AnalyticsClient2` | analytics operations |
| `client.requests` | `elevenlabs_sdk::api::RequestsClient` | requests operations |
| `client.service_accounts` | `elevenlabs_sdk::api::ServiceAccountsClient` | service_accounts operations |
| `client.api_keys` | `elevenlabs_sdk::api::ApiKeysClient` | api_keys operations |
| `client.webhooks` | `elevenlabs_sdk::api::WebhooksClient` | webhooks operations |
| `client.music` | `elevenlabs_sdk::api::MusicClient` | music operations |
| `client.composition_plan` | `elevenlabs_sdk::api::CompositionPlanClient` | composition_plan operations |
| `client.finetunes` | `elevenlabs_sdk::api::FinetunesClient` | finetunes operations |
| `client.speech_to_text` | `elevenlabs_sdk::api::SpeechToTextClient` | speech_to_text operations |
| `client.transcripts` | `elevenlabs_sdk::api::TranscriptsClient2` | transcripts operations |
| `client.forced_alignment` | `elevenlabs_sdk::api::ForcedAlignmentClient` | forced_alignment operations |
| `client.agents` | `elevenlabs_sdk::api::AgentsClient` | agents operations |
| `client.conversations` | `elevenlabs_sdk::api::ConversationsClient` | conversations operations |
| `client.audio` | `elevenlabs_sdk::api::AudioClient` | audio operations |
| `client.feedback` | `elevenlabs_sdk::api::FeedbackClient` | feedback operations |
| `client.messages` | `elevenlabs_sdk::api::MessagesClient` | messages operations |
| `client.tags` | `elevenlabs_sdk::api::TagsClient` | tags operations |
| `client.files` | `elevenlabs_sdk::api::FilesClient` | files operations |
| `client.topics` | `elevenlabs_sdk::api::TopicsClient` | topics operations |
| `client.analysis` | `elevenlabs_sdk::api::AnalysisClient` | analysis operations |
| `client.twilio` | `elevenlabs_sdk::api::TwilioClient` | twilio operations |
| `client.exotel` | `elevenlabs_sdk::api::ExotelClient` | exotel operations |
| `client.whatsapp` | `elevenlabs_sdk::api::WhatsappClient` | whatsapp operations |
| `client.summaries` | `elevenlabs_sdk::api::SummariesClient` | summaries operations |
| `client.widget` | `elevenlabs_sdk::api::WidgetClient` | widget operations |
| `client.avatar` | `elevenlabs_sdk::api::AvatarClient` | avatar operations |
| `client.link` | `elevenlabs_sdk::api::LinkClient` | link operations |
| `client.knowledge_base` | `elevenlabs_sdk::api::KnowledgeBaseClient` | knowledge_base operations |
| `client.documents` | `elevenlabs_sdk::api::DocumentsClient` | documents operations |
| `client.summaries` | `elevenlabs_sdk::api::SummariesClient2` | summaries operations |
| `client.chunk` | `elevenlabs_sdk::api::ChunkClient` | chunk operations |
| `client.chunks` | `elevenlabs_sdk::api::ChunksClient` | chunks operations |
| `client.crawl_jobs` | `elevenlabs_sdk::api::CrawlJobsClient` | crawl_jobs operations |
| `client.document` | `elevenlabs_sdk::api::DocumentClient` | document operations |
| `client.tests` | `elevenlabs_sdk::api::TestsClient` | tests operations |
| `client.folders` | `elevenlabs_sdk::api::FoldersClient` | folders operations |
| `client.invocations` | `elevenlabs_sdk::api::InvocationsClient` | invocations operations |
| `client.users` | `elevenlabs_sdk::api::UsersClient` | users operations |
| `client.phone_numbers` | `elevenlabs_sdk::api::PhoneNumbersClient` | phone_numbers operations |
| `client.llm_usage` | `elevenlabs_sdk::api::LlmUsageClient` | llm_usage operations |
| `client.llm` | `elevenlabs_sdk::api::LlmClient` | llm operations |
| `client.tools` | `elevenlabs_sdk::api::ToolsClient` | tools operations |
| `client.executions` | `elevenlabs_sdk::api::ExecutionsClient` | executions operations |
| `client.settings` | `elevenlabs_sdk::api::SettingsClient` | settings operations |
| `client.secrets` | `elevenlabs_sdk::api::SecretsClient` | secrets operations |
| `client.batch_calls` | `elevenlabs_sdk::api::BatchCallsClient` | batch_calls operations |
| `client.sip_trunk` | `elevenlabs_sdk::api::SipTrunkClient` | sip_trunk operations |
| `client.mcp_servers` | `elevenlabs_sdk::api::McpServersClient` | mcp_servers operations |
| `client.tools` | `elevenlabs_sdk::api::ToolsClient2` | tools operations |
| `client.approval_policy` | `elevenlabs_sdk::api::ApprovalPolicyClient` | approval_policy operations |
| `client.tool_approvals` | `elevenlabs_sdk::api::ToolApprovalsClient` | tool_approvals operations |
| `client.tool_configs` | `elevenlabs_sdk::api::ToolConfigsClient` | tool_configs operations |
| `client.whatsapp_accounts` | `elevenlabs_sdk::api::WhatsappAccountsClient` | whatsapp_accounts operations |
| `client.branches` | `elevenlabs_sdk::api::BranchesClient` | branches operations |
| `client.versions` | `elevenlabs_sdk::api::VersionsClient` | versions operations |
| `client.deployments` | `elevenlabs_sdk::api::DeploymentsClient` | deployments operations |
| `client.drafts` | `elevenlabs_sdk::api::DraftsClient` | drafts operations |
| `client.agents` | `elevenlabs_sdk::api::AgentsClient2` | agents operations |
| `client.llm_usage` | `elevenlabs_sdk::api::LlmUsageClient2` | llm_usage operations |
| `client.analytics` | `elevenlabs_sdk::api::AnalyticsClient` | analytics operations |
| `client.live_count` | `elevenlabs_sdk::api::LiveCountClient` | live_count operations |
| `client.dashboard` | `elevenlabs_sdk::api::DashboardClient` | dashboard operations |
| `client.settings` | `elevenlabs_sdk::api::SettingsClient2` | settings operations |
| `client.speech_engine` | `elevenlabs_sdk::api::SpeechEngineClient` | speech_engine operations |
| `client.environment_variables` | `elevenlabs_sdk::api::EnvironmentVariablesClient` | environment_variables operations |
| `client.productions` | `elevenlabs_sdk::api::ProductionsClient` | productions operations |
| `client.orders` | `elevenlabs_sdk::api::OrdersClient` | orders operations |
| `client.media` | `elevenlabs_sdk::api::MediaClient` | media operations |
| `client.items` | `elevenlabs_sdk::api::ItemsClient` | items operations |
| `client.deliverables` | `elevenlabs_sdk::api::DeliverablesClient` | deliverables operations |
| `client.languages` | `elevenlabs_sdk::api::LanguagesClient` | languages operations |
| `client.tokens` | `elevenlabs_sdk::api::TokensClient` | tokens operations |
| `client.single_use` | `elevenlabs_sdk::api::SingleUseClient` | single_use operations |
| `client.workspaces` | `elevenlabs_sdk::api::WorkspacesClient` | workspaces operations |
| `client.api_keys` | `elevenlabs_sdk::api::ApiKeysClient2` | api_keys operations |

### 3. Key Patterns

**Get the SDK client** (execution-sharing, fully authenticated):
```rust
let client = super::sdk::client(ctx);
```

**Run an async SDK call from a sync handler:**
```rust
let result = super::sdk::block_on(
    client.some_resource.some_method(args),
)?;
```

**Use typed models for request/response serialization:**
```rust
use elevenlabs_sdk::api::*;
```

## Regeneration Safety

| File | Regenerated? | Notes |
|------|-------------|-------|
| `cli/elevenlabs/custom.rs` | **No** | Protected by `.fernignore` |
| `cli/elevenlabs/sdk.rs` | Yes | Bridges AppContext → SDK client |
| `cli/elevenlabs/main.rs` | Yes | Calls `custom::register(app)` |
| `elevenlabs-sdk/` | Yes | Co-generated typed SDK crate |
| `elevenlabs-types/` | Yes | Co-generated typed models |

After running `fern generate`, your `custom.rs` is preserved. All
generated code (SDK, types, glue, main.rs) is updated to match the
latest API spec. If the SDK surface changes (renamed methods, new
sub-clients), update your `custom.rs` to match.

## Build & Test

```bash
# Build the CLI (includes custom commands)
cargo build

# Run your custom command
elevenlabs <your-command> [args]

# Run with verbose output for debugging
RUST_LOG=debug elevenlabs <your-command> [args]
```
