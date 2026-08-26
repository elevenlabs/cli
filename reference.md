# ElevenLabs API Documentation CLI Reference

Full command reference for `elevenlabs`.

## Commands

- [`elevenlabs agents`](#elevenlabs-agents)
- [`elevenlabs agents agents llm-usage`](#elevenlabs-agents-agents-llm-usage)
- [`elevenlabs agents analytics live-count`](#elevenlabs-agents-analytics-live-count)
- [`elevenlabs agents batch-calls`](#elevenlabs-agents-batch-calls)
- [`elevenlabs agents branches`](#elevenlabs-agents-branches)
- [`elevenlabs agents conversations`](#elevenlabs-agents-conversations)
- [`elevenlabs agents conversations analysis`](#elevenlabs-agents-conversations-analysis)
- [`elevenlabs agents conversations audio`](#elevenlabs-agents-conversations-audio)
- [`elevenlabs agents conversations feedback`](#elevenlabs-agents-conversations-feedback)
- [`elevenlabs agents conversations files`](#elevenlabs-agents-conversations-files)
- [`elevenlabs agents conversations messages`](#elevenlabs-agents-conversations-messages)
- [`elevenlabs agents conversations tags`](#elevenlabs-agents-conversations-tags)
- [`elevenlabs agents conversations topics`](#elevenlabs-agents-conversations-topics)
- [`elevenlabs agents dashboard settings`](#elevenlabs-agents-dashboard-settings)
- [`elevenlabs agents deployments`](#elevenlabs-agents-deployments)
- [`elevenlabs agents drafts`](#elevenlabs-agents-drafts)
- [`elevenlabs agents exotel`](#elevenlabs-agents-exotel)
- [`elevenlabs agents knowledge-base`](#elevenlabs-agents-knowledge-base)
- [`elevenlabs agents knowledge-base crawl-jobs`](#elevenlabs-agents-knowledge-base-crawl-jobs)
- [`elevenlabs agents knowledge-base document`](#elevenlabs-agents-knowledge-base-document)
- [`elevenlabs agents knowledge-base documents`](#elevenlabs-agents-knowledge-base-documents)
- [`elevenlabs agents knowledge-base documents chunk`](#elevenlabs-agents-knowledge-base-documents-chunk)
- [`elevenlabs agents knowledge-base documents chunks`](#elevenlabs-agents-knowledge-base-documents-chunks)
- [`elevenlabs agents knowledge-base documents summaries`](#elevenlabs-agents-knowledge-base-documents-summaries)
- [`elevenlabs agents link`](#elevenlabs-agents-link)
- [`elevenlabs agents llm`](#elevenlabs-agents-llm)
- [`elevenlabs agents llm-usage`](#elevenlabs-agents-llm-usage)
- [`elevenlabs agents mcp-servers`](#elevenlabs-agents-mcp-servers)
- [`elevenlabs agents mcp-servers approval-policy`](#elevenlabs-agents-mcp-servers-approval-policy)
- [`elevenlabs agents mcp-servers tool-approvals`](#elevenlabs-agents-mcp-servers-tool-approvals)
- [`elevenlabs agents mcp-servers tool-configs`](#elevenlabs-agents-mcp-servers-tool-configs)
- [`elevenlabs agents mcp-servers tools`](#elevenlabs-agents-mcp-servers-tools)
- [`elevenlabs agents phone-numbers`](#elevenlabs-agents-phone-numbers)
- [`elevenlabs agents procedures`](#elevenlabs-agents-procedures)
- [`elevenlabs agents procedures drafts`](#elevenlabs-agents-procedures-drafts)
- [`elevenlabs agents secrets`](#elevenlabs-agents-secrets)
- [`elevenlabs agents settings`](#elevenlabs-agents-settings)
- [`elevenlabs agents sip-trunk`](#elevenlabs-agents-sip-trunk)
- [`elevenlabs agents summaries`](#elevenlabs-agents-summaries)
- [`elevenlabs agents tests`](#elevenlabs-agents-tests)
- [`elevenlabs agents tests folders`](#elevenlabs-agents-tests-folders)
- [`elevenlabs agents tests invocations`](#elevenlabs-agents-tests-invocations)
- [`elevenlabs agents tools`](#elevenlabs-agents-tools)
- [`elevenlabs agents tools executions`](#elevenlabs-agents-tools-executions)
- [`elevenlabs agents triage-tickets`](#elevenlabs-agents-triage-tickets)
- [`elevenlabs agents twilio`](#elevenlabs-agents-twilio)
- [`elevenlabs agents users`](#elevenlabs-agents-users)
- [`elevenlabs agents versions`](#elevenlabs-agents-versions)
- [`elevenlabs agents whatsapp`](#elevenlabs-agents-whatsapp)
- [`elevenlabs agents whatsapp-accounts`](#elevenlabs-agents-whatsapp-accounts)
- [`elevenlabs agents widget`](#elevenlabs-agents-widget)
- [`elevenlabs agents widget avatar`](#elevenlabs-agents-widget-avatar)
- [`elevenlabs assets`](#elevenlabs-assets)
- [`elevenlabs audio-isolation`](#elevenlabs-audio-isolation)
- [`elevenlabs audio-native`](#elevenlabs-audio-native)
- [`elevenlabs dubbing`](#elevenlabs-dubbing)
- [`elevenlabs dubbing audio`](#elevenlabs-dubbing-audio)
- [`elevenlabs dubbing project`](#elevenlabs-dubbing-project)
- [`elevenlabs dubbing project language`](#elevenlabs-dubbing-project-language)
- [`elevenlabs dubbing project language transcript`](#elevenlabs-dubbing-project-language-transcript)
- [`elevenlabs dubbing project transcript`](#elevenlabs-dubbing-project-transcript)
- [`elevenlabs dubbing resource`](#elevenlabs-dubbing-resource)
- [`elevenlabs dubbing resource language`](#elevenlabs-dubbing-resource-language)
- [`elevenlabs dubbing resource segment`](#elevenlabs-dubbing-resource-segment)
- [`elevenlabs dubbing resource speaker`](#elevenlabs-dubbing-resource-speaker)
- [`elevenlabs dubbing resource speaker segment`](#elevenlabs-dubbing-resource-speaker-segment)
- [`elevenlabs dubbing transcript`](#elevenlabs-dubbing-transcript)
- [`elevenlabs dubbing transcripts`](#elevenlabs-dubbing-transcripts)
- [`elevenlabs environment-variables`](#elevenlabs-environment-variables)
- [`elevenlabs flows image`](#elevenlabs-flows-image)
- [`elevenlabs flows text-to-speech`](#elevenlabs-flows-text-to-speech)
- [`elevenlabs flows video`](#elevenlabs-flows-video)
- [`elevenlabs forced-alignment`](#elevenlabs-forced-alignment)
- [`elevenlabs history`](#elevenlabs-history)
- [`elevenlabs models`](#elevenlabs-models)
- [`elevenlabs music`](#elevenlabs-music)
- [`elevenlabs music composition-plan`](#elevenlabs-music-composition-plan)
- [`elevenlabs music finetunes`](#elevenlabs-music-finetunes)
- [`elevenlabs productions orders`](#elevenlabs-productions-orders)
- [`elevenlabs productions orders deliverables`](#elevenlabs-productions-orders-deliverables)
- [`elevenlabs productions orders items`](#elevenlabs-productions-orders-items)
- [`elevenlabs productions orders languages`](#elevenlabs-productions-orders-languages)
- [`elevenlabs productions orders media`](#elevenlabs-productions-orders-media)
- [`elevenlabs pronunciation-dictionaries`](#elevenlabs-pronunciation-dictionaries)
- [`elevenlabs pronunciation-dictionaries rules`](#elevenlabs-pronunciation-dictionaries-rules)
- [`elevenlabs samples`](#elevenlabs-samples)
- [`elevenlabs service-accounts`](#elevenlabs-service-accounts)
- [`elevenlabs service-accounts api-keys`](#elevenlabs-service-accounts-api-keys)
- [`elevenlabs speech-engine`](#elevenlabs-speech-engine)
- [`elevenlabs speech-to-speech`](#elevenlabs-speech-to-speech)
- [`elevenlabs speech-to-text`](#elevenlabs-speech-to-text)
- [`elevenlabs speech-to-text transcripts`](#elevenlabs-speech-to-text-transcripts)
- [`elevenlabs studio`](#elevenlabs-studio)
- [`elevenlabs studio projects`](#elevenlabs-studio-projects)
- [`elevenlabs studio projects chapters`](#elevenlabs-studio-projects-chapters)
- [`elevenlabs studio projects chapters snapshots`](#elevenlabs-studio-projects-chapters-snapshots)
- [`elevenlabs studio projects content`](#elevenlabs-studio-projects-content)
- [`elevenlabs studio projects pronunciation-dictionaries`](#elevenlabs-studio-projects-pronunciation-dictionaries)
- [`elevenlabs studio projects snapshots`](#elevenlabs-studio-projects-snapshots)
- [`elevenlabs text-to-dialogue`](#elevenlabs-text-to-dialogue)
- [`elevenlabs text-to-sound-effects`](#elevenlabs-text-to-sound-effects)
- [`elevenlabs text-to-speech`](#elevenlabs-text-to-speech)
- [`elevenlabs text-to-voice`](#elevenlabs-text-to-voice)
- [`elevenlabs text-to-voice preview`](#elevenlabs-text-to-voice-preview)
- [`elevenlabs tokens single-use`](#elevenlabs-tokens-single-use)
- [`elevenlabs usage`](#elevenlabs-usage)
- [`elevenlabs user`](#elevenlabs-user)
- [`elevenlabs user subscription`](#elevenlabs-user-subscription)
- [`elevenlabs voices`](#elevenlabs-voices)
- [`elevenlabs voices accents`](#elevenlabs-voices-accents)
- [`elevenlabs voices ivc`](#elevenlabs-voices-ivc)
- [`elevenlabs voices pvc`](#elevenlabs-voices-pvc)
- [`elevenlabs voices pvc samples`](#elevenlabs-voices-pvc-samples)
- [`elevenlabs voices pvc samples audio`](#elevenlabs-voices-pvc-samples-audio)
- [`elevenlabs voices pvc samples speakers`](#elevenlabs-voices-pvc-samples-speakers)
- [`elevenlabs voices pvc samples speakers audio`](#elevenlabs-voices-pvc-samples-speakers-audio)
- [`elevenlabs voices pvc samples waveform`](#elevenlabs-voices-pvc-samples-waveform)
- [`elevenlabs voices pvc verification`](#elevenlabs-voices-pvc-verification)
- [`elevenlabs voices pvc verification captcha`](#elevenlabs-voices-pvc-verification-captcha)
- [`elevenlabs voices samples audio`](#elevenlabs-voices-samples-audio)
- [`elevenlabs voices settings`](#elevenlabs-voices-settings)
- [`elevenlabs webhooks`](#elevenlabs-webhooks)
- [`elevenlabs workspace`](#elevenlabs-workspace)
- [`elevenlabs workspace analytics requests`](#elevenlabs-workspace-analytics-requests)
- [`elevenlabs workspace audit-logs`](#elevenlabs-workspace-audit-logs)
- [`elevenlabs workspace auth-connections`](#elevenlabs-workspace-auth-connections)
- [`elevenlabs workspace groups`](#elevenlabs-workspace-groups)
- [`elevenlabs workspace groups members`](#elevenlabs-workspace-groups-members)
- [`elevenlabs workspace invites`](#elevenlabs-workspace-invites)
- [`elevenlabs workspace members`](#elevenlabs-workspace-members)
- [`elevenlabs workspace resources`](#elevenlabs-workspace-resources)
- [`elevenlabs workspace usage`](#elevenlabs-workspace-usage)
- [`elevenlabs workspaces api-keys`](#elevenlabs-workspaces-api-keys)

---

### `elevenlabs agents`

#### `elevenlabs agents add-to-knowledge-base` `[DEPRECATED]`

Upload a file or webpage URL to create a knowledge base document. <br> <Note> After creating the document, update the agent's knowledge base by calling [Update agent](/docs/api-reference/agents/update). </Note>

`POST /v1/convai/knowledge-base`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | No |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents create`

Create an agent from a config object

`POST /v1/convai/agents/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--enable-versioning` | `boolean` | No | Deprecated: all agents are versioned. This parameter is ignored. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents delete`

Delete an agent

`DELETE /v1/convai/agents/{agent_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents delete-document-rag-index`

Delete RAG index for the knowledgebase document.

`DELETE /v1/convai/knowledge-base/{documentation_id}/rag-index/{rag_index_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--rag-index-id` | `string` | Yes | The id of RAG index of document from the knowledge base. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents duplicate`

Create a new agent by duplicating an existing one

`POST /v1/convai/agents/{agent_id}/duplicate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents get`

Retrieve config for an agent

`GET /v1/convai/agents/{agent_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--version-id` | `string` | No | The ID of the agent version to use |
| `--branch-id` | `string` | No | The ID of the branch to use |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents get-document-rag-indexes`

Provides information about all RAG indexes of the specified knowledgebase document.

`GET /v1/convai/knowledge-base/{documentation_id}/rag-index`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents list`

Returns a list of your agents and their metadata.

`GET /v1/convai/agents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-size` | `integer` | No | How many Agents to return at maximum. Can not exceed 100, defaults to 30. |
| `--search` | `string` | No | Search by agents name. |
| `--archived` | `string` | No | Filter agents by archived status |
| `--show-only-owned-agents` | `boolean` | No | If set to true, the endpoint will omit any agents that were shared with you by someone else and include only the ones you own. Deprecated: use created_by_user_id instead. |
| `--created-by-user-id` | `string` | No | Filter agents by creator user ID. When set, only agents created by this user are returned. Takes precedence over show_only_owned_agents. Use '@me' to refer to the authenticated user. |
| `--sort-direction` | `SortDirection` | No | The direction to sort the results |
| `--sort-by` | `string` | No | The field to sort the results by |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents rag-index-overview`

Provides total size and other information of RAG indexes used by knowledgebase documents

`GET /v1/convai/knowledge-base/rag-index`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents run-tests`

Run selected tests on the agent with provided configuration. If the agent configuration is provided, it will be used to override default agent configuration.

`POST /v1/convai/agents/{agent_id}/run-tests`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents simulate-conversation` `[DEPRECATED]`

Deprecated. Use the `/v1/convai/agent-testing/create` and `/v1/convai/agents/:agent_id/run-tests` endpoints to create and run simulations. Run a conversation between the agent and a simulated user.

`POST /v1/convai/agents/{agent_id}/simulate-conversation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents simulate-conversation-stream` `[DEPRECATED]`

Deprecated. Use the `/v1/convai/agent-testing/create` and `/v1/convai/agents/:agent_id/run-tests` endpoints to create and run simulations. Run a conversation between the agent and a simulated user and stream back the response. Response is streamed back as partial lists of messages that should be concatenated and once the conversation has complete a single final message with the conversation analysis will be sent.

`POST /v1/convai/agents/{agent_id}/simulate-conversation/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents update`

Patches an Agent settings

`PATCH /v1/convai/agents/{agent_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--enable-versioning-if-not-enabled` | `boolean` | No | Deprecated: all agents are versioned. This parameter is ignored. |
| `--branch-id` | `string` | No | The ID of the branch to use |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents agents llm-usage`

#### `elevenlabs agents agents llm-usage calculate`

Calculates expected number of LLM tokens needed for the specified agent.

`POST /v1/convai/agent/{agent_id}/llm-usage/calculate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents analytics live-count`

#### `elevenlabs agents analytics live-count get`

Get the live count of the ongoing conversations.

`GET /v1/convai/analytics/live-count`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | No | The id of an agent to restrict the analytics to. |
| `--agent-ids` | `string` | No | Restrict analytics to the union of the given agents. Takes precedence over `agent_id` when both are supplied. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents batch-calls`

#### `elevenlabs agents batch-calls cancel`

Cancel a running batch call and set all recipients to cancelled status.

`POST /v1/convai/batch-calling/{batch_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--batch-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents batch-calls create`

Submit a batch call request to schedule calls for multiple recipients.

`POST /v1/convai/batch-calling/submit`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents batch-calls delete`

Permanently delete a batch call and all recipient records. Conversations remain in history.

`DELETE /v1/convai/batch-calling/{batch_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--batch-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents batch-calls export`

Download all recipients and conversation results for a terminal batch call as CSV.

`GET /v1/convai/batch-calling/{batch_id}/export`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--batch-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents batch-calls get`

Get detailed information about a batch call including all recipients.

`GET /v1/convai/batch-calling/{batch_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--batch-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents batch-calls list`

Get all batch calls for the current workspace.

`GET /v1/convai/batch-calling/workspace`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No |  |
| `--last-doc` | `string` | No |  |
| `--agent-id` | `string` | No | Filter batch calls to a single agent. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents batch-calls retry`

Retry a batch call, calling failed and no-response recipients again.

`POST /v1/convai/batch-calling/{batch_id}/retry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--batch-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents branches`

#### `elevenlabs agents branches create`

Create a new branch from a given version of any branch

`POST /v1/convai/agents/{agent_id}/branches`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents branches get`

Get information about a single agent branch

`GET /v1/convai/agents/{agent_id}/branches/{branch_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--branch-id` | `string` | Yes | Unique identifier for the branch. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents branches list`

Returns a list of branches an agent has

`GET /v1/convai/agents/{agent_id}/branches`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--include-archived` | `boolean` | No | Whether archived branches should be included |
| `--limit` | `integer` | No | How many results at most should be returned |
| `--include-commit-status` | `boolean` | No | Whether to compute how far each branch has diverged from main (commits_ahead/commits_behind). This walks the version DAG of every branch, so it is slow on agents with long histories and is off by default, leaving those fields null. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents branches merge`

Merge a branch into a target branch

`POST /v1/convai/agents/{agent_id}/branches/{source_branch_id}/merge`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--source-branch-id` | `string` | Yes | Unique identifier for the source branch to merge from. |
| `--target-branch-id` | `string` | Yes | The ID of the target branch to merge into. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents branches preview-merge`

Returns the result of merging the source branch into the target branch without performing the merge. Useful for showing an accurate diff before confirming.

`GET /v1/convai/agents/{agent_id}/branches/{source_branch_id}/merge-preview`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--source-branch-id` | `string` | Yes | Unique identifier for the source branch to merge from. |
| `--target-branch-id` | `string` | Yes | The ID of the target branch to merge into. |
| `--force` | `boolean` | No | When true, source branch changes always win conflicts regardless of timestamps |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents branches preview-rebase`

Returns the result of rebasing the branch onto main without performing the rebase. Useful for showing an accurate diff before confirming.

`GET /v1/convai/agents/{agent_id}/branches/{branch_id}/rebase-preview`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--branch-id` | `string` | Yes | Unique identifier for the source branch to merge from. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents branches rebase`

Rebase a branch onto the latest main branch, incorporating main's changes while preserving the branch's own changes.

`POST /v1/convai/agents/{agent_id}/branches/{branch_id}/rebase`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--branch-id` | `string` | Yes | Unique identifier for the source branch to merge from. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents branches update`

Update agent branch properties such as archiving status and protection level

`PATCH /v1/convai/agents/{agent_id}/branches/{branch_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--branch-id` | `string` | Yes | Unique identifier for the branch. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents conversations`

#### `elevenlabs agents conversations delete`

Delete a particular conversation

`DELETE /v1/convai/conversations/{conversation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes | The id of the conversation you're taking the action on. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations get`

Get the details of a particular conversation

`GET /v1/convai/conversations/{conversation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes | The id of the conversation you're taking the action on. |
| `--format` | `json | opentelemetry` | No | Response format. Defaults to 'json'. Set to 'opentelemetry' for an OTLP-compatible trace payload using the same structure as the post-call webhook. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations get-signed-url`

Get a signed url to start a conversation with an agent with an agent that requires authorization

`GET /v1/convai/conversation/get-signed-url`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource. |
| `--include-conversation-id` | `boolean` | No | Whether to include a conversation_id with the response. If included, the conversation_signature cannot be used again. |
| `--branch-id` | `string` | No | The ID of the branch to use |
| `--environment` | `string` | No | The environment to use for resolving environment variables (e.g. 'production', 'staging'). Defaults to 'production'. |
| `--debug-events-request` | `boolean` | No | Whether to enable debug events. Only available for users with editor access to the agent. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations get-sip-messages`

Get SIP messages associated with a conversation's phone call

`GET /v1/convai/conversations/{conversation_id}/sip-messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes | The id of the conversation you're taking the action on. |
| `--page-size` | `integer` | No |  |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations get-summary`

Get a lightweight summary of a conversation: its title, the generated transcript summary, whether the call was successful, and — only when the conversation is short — the plain chat messages. Tool calls, tool results, and contextual updates are omitted so the response stays small. Use this instead of the full conversation endpoint when you only need the gist (e.g. an agent reading many conversations); use GET /v1/convai/conversations/{conversation_id} when you need the full transcript with tool calls and contextual updates.

`GET /v1/convai/conversations/{conversation_id}/summary`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes | The id of the conversation you're taking the action on. |
| `--max-messages` | `integer` | No | Maximum number of chat message turns to include inline. When the conversation has more than this, the messages are omitted and messages_omitted is set. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations get-webrtc-token`

Get a WebRTC session token for real-time communication.

`GET /v1/convai/conversation/token`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource. |
| `--participant-name` | `string` | No | Optional custom participant name. If not provided, user ID will be used |
| `--branch-id` | `string` | No | The ID of the branch to use |
| `--environment` | `string` | No | The environment to use for resolving environment variables (e.g. 'production', 'staging'). Defaults to 'production'. |
| `--debug-events-request` | `boolean` | No | Whether to enable debug events. Only available for users with editor access to the agent. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations list`

Get all conversations of agents that user owns. With option to restrict to a specific agent.

`GET /v1/convai/conversations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--agent-id` | `string` | No | Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource. |
| `--visited-agent-ids` | `string` | No | Filter conversations where any of these agents participated. Can not exceed 50 values. |
| `--visited-agent-branch-ids` | `string` | No | Filter conversations where any of these agent branches participated. Can not exceed 50 values. |
| `--triggered-procedure-ids` | `string` | No | Filter conversations where any of these procedures were triggered. Can not exceed 50 values. |
| `--call-successful` | `string` | No | The result of the success evaluation |
| `--call-start-before-unix` | `string` | No | Unix timestamp (in seconds) to filter conversations up to this start date. |
| `--call-start-after-unix` | `string` | No | Unix timestamp (in seconds) to filter conversations after to this start date. |
| `--call-duration-min-secs` | `string` | No | Minimum call duration in seconds. |
| `--call-duration-max-secs` | `string` | No | Maximum call duration in seconds. |
| `--rating-max` | `string` | No | Maximum overall rating (1-5). |
| `--rating-min` | `string` | No | Minimum overall rating (1-5). |
| `--has-feedback-comment` | `string` | No | Filter conversations with user feedback comments. |
| `--user-id` | `string` | No | Filter conversations by the user ID who initiated them. |
| `--evaluation-params` | `string` | No | Evaluation filters. Repeat param. Format: criteria_id:result. Example: eval=value_framing:success |
| `--data-collection-params` | `string` | No | Data collection filters. Repeat param. Format: id:op:value where op is one of eq\|neq\|gt\|gte\|lt\|lte\|in\|exists\|missing. For in, pipe-delimit values. |
| `--data-collection-ids` | `string` | No | Data collection field IDs to include in each conversation summary. Repeat param. When omitted, data_collection_results is not returned. |
| `--evaluation-criteria-ids` | `string` | No | Evaluation criteria IDs to include in each conversation summary. Repeat param. When omitted, evaluation_criteria_results is not returned. |
| `--tool-names` | `string` | No | Filter conversations by tool names used during the call. |
| `--tool-names-successful` | `string` | No | Filter conversations by tool names that had successful calls. |
| `--tool-names-errored` | `string` | No | Filter conversations by tool names that had errored calls. |
| `--include-invalid-tool-calls` | `boolean` | No | Also match tool calls that never ran. |
| `--main-languages` | `string` | No | Filter conversations by detected main language (language code). |
| `--page-size` | `integer` | No | How many conversations to return at maximum. Can not exceed 100, defaults to 30. |
| `--summary-mode` | `exclude | include` | No | Whether to include transcript summaries in the response. |
| `--search` | `string` | No | Full-text or fuzzy search over transcript messages |
| `--conversation-initiation-source` | `string` | No |  |
| `--text-only` | `string` | No |  |
| `--conversation-product-type` | `string` | No | Restrict results to a single conversation product surface. |
| `--branch-id` | `string` | No | Filter conversations by branch ID. |
| `--version-id` | `string` | No | Filter conversations by version ID. |
| `--parent-conversation-id` | `string` | No | Filter conversations by parent conversation ID for subagent conversations. |
| `--topic-ids` | `string` | No | Filter conversations by topic IDs assigned during topic discovery. |
| `--exclude-statuses` | `string` | No | Exclude conversations with the given statuses. Useful for hiding in-progress / processing conversations from list views. |
| `--tag-ids` | `string` | No | Filter conversations by conversation tag IDs assigned via the conversation-tags endpoints. |
| `--workflow-node-entered-id` | `string` | No | Filter conversations to only those that entered the given node. |
| `--termination-reasons` | `string` | No | Filter conversations by their stored termination_reason (metadata.termination_reason). Repeat param to match any of several. |
| `--guardrail-types` | `string` | No | Filter to conversations where a guardrail of any of these types triggered (metadata.triggered_guardrails.guardrail_type). Repeat param to match any of several. |
| `--custom-guardrail-names` | `string` | No | Filter to conversations where a custom guardrail with any of these names triggered (metadata.triggered_guardrails.guardrail_name). Only custom guardrails carry a name. Repeat param to match any of several. |
| `--sort-direction` | `SortDirection` | No | The direction to sort conversations by call start time. Defaults to descending (newest first). |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations resolve`

Resolve a conversation URL (a Slack message URL or a Zendesk ticket URL) to the deterministic conversation ID for the given agent, then confirm the conversation exists.

`GET /v1/convai/conversations/resolve`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource. |
| `--reference` | `string` | Yes | A Slack message URL or a Zendesk ticket URL. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents conversations analysis`

#### `elevenlabs agents conversations analysis run`

Run the analysis for a conversation using the agent's current evaluation criteria and data collection settings.

`POST /v1/convai/conversations/{conversation_id}/analysis/run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes | ID of the conversation |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations analysis run-evaluation`

Rerun a specific evaluation for a conversation.

`POST /v1/convai/conversations/{conversation_id}/analysis/evaluations/run`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes | ID of the conversation |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents conversations audio`

#### `elevenlabs agents conversations audio get`

Get the audio recording of a particular conversation

`GET /v1/convai/conversations/{conversation_id}/audio`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes | The id of the conversation you're taking the action on. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents conversations feedback`

#### `elevenlabs agents conversations feedback create`

Send the feedback for the given conversation

`POST /v1/convai/conversations/{conversation_id}/feedback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes | The id of the conversation you're taking the action on. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents conversations files`

#### `elevenlabs agents conversations files create`

Upload an image or PDF file for a conversation. Returns a unique file ID that can be used to reference the file in the conversation.

`POST /v1/convai/conversations/{conversation_id}/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents conversations files delete`

Remove a file upload from a conversation. Only possible if the file hasn't already been used in the conversation.

`DELETE /v1/convai/conversations/{conversation_id}/files/{file_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--file-id` | `string` | Yes |  |
| `--conversation-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents conversations messages`

#### `elevenlabs agents conversations messages search`

Search conversation transcripts by semantic similarity to surface relevant messages based on meaning and intent, rather than exact keyword matches

`GET /v1/convai/conversations/messages/smart-search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--text-query` | `string` | Yes | The search query text for semantic similarity matching |
| `--agent-id` | `string` | No | Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource. |
| `--page-size` | `integer` | No | Number of results per page. Max 50. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations messages text-search`

Search through conversation transcript messages by full-text and fuzzy search

`GET /v1/convai/conversations/messages/text-search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--text-query` | `string` | Yes | The search query text for full-text and fuzzy matching |
| `--agent-id` | `string` | No | Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource. |
| `--visited-agent-ids` | `string` | No | Filter conversations where any of these agents participated. Can not exceed 50 values. |
| `--visited-agent-branch-ids` | `string` | No | Filter conversations where any of these agent branches participated. Can not exceed 50 values. |
| `--triggered-procedure-ids` | `string` | No | Filter conversations where any of these procedures were triggered. Can not exceed 50 values. |
| `--call-successful` | `string` | No | The result of the success evaluation |
| `--call-start-before-unix` | `string` | No | Unix timestamp (in seconds) to filter conversations up to this start date. |
| `--call-start-after-unix` | `string` | No | Unix timestamp (in seconds) to filter conversations after to this start date. |
| `--call-duration-min-secs` | `string` | No | Minimum call duration in seconds. |
| `--call-duration-max-secs` | `string` | No | Maximum call duration in seconds. |
| `--rating-max` | `string` | No | Maximum overall rating (1-5). |
| `--rating-min` | `string` | No | Minimum overall rating (1-5). |
| `--has-feedback-comment` | `string` | No | Filter conversations with user feedback comments. |
| `--user-id` | `string` | No | Filter conversations by the user ID who initiated them. |
| `--evaluation-params` | `string` | No | Evaluation filters. Repeat param. Format: criteria_id:result. Example: eval=value_framing:success |
| `--data-collection-params` | `string` | No | Data collection filters. Repeat param. Format: id:op:value where op is one of eq\|neq\|gt\|gte\|lt\|lte\|in\|exists\|missing. For in, pipe-delimit values. |
| `--tool-names` | `string` | No | Filter conversations by tool names used during the call. |
| `--tool-names-successful` | `string` | No | Filter conversations by tool names that had successful calls. |
| `--tool-names-errored` | `string` | No | Filter conversations by tool names that had errored calls. |
| `--include-invalid-tool-calls` | `boolean` | No | Also match tool calls that never ran. |
| `--main-languages` | `string` | No | Filter conversations by detected main language (language code). |
| `--exclude-statuses` | `string` | No | Exclude conversations with the given statuses. Useful for hiding in-progress / processing conversations from list views. |
| `--termination-reasons` | `string` | No | Filter conversations by their stored termination_reason (metadata.termination_reason). Repeat param to match any of several. |
| `--page-size` | `integer` | No | Number of results per page. Max 50. |
| `--summary-mode` | `exclude | include` | No | Whether to include transcript summaries in the response. |
| `--conversation-initiation-source` | `string` | No |  |
| `--text-only` | `string` | No |  |
| `--conversation-product-type` | `string` | No | Restrict results to a single conversation product surface. |
| `--branch-id` | `string` | No | Filter conversations by branch ID. |
| `--version-id` | `string` | No | Filter conversations by version ID. |
| `--topic-ids` | `string` | No | Filter conversations by topic IDs assigned during topic discovery. |
| `--sort-by` | `MessageSearchSortBy` | No | Sort order for search results. 'search_score' sorts by search score, 'created_at' sorts by conversation start time. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents conversations tags`

#### `elevenlabs agents conversations tags assign`

Assign one or more conversation tags to a conversation. Tags that are already assigned are ignored. Tags must belong to the same workspace.

`POST /v1/convai/conversations/{conversation_id}/tags`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents conversations tags create`

Create a new conversation tag for the workspace.

`POST /v1/convai/tags`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents conversations tags delete`

Delete a conversation tag. Restricted to the tag owner or a workspace admin.

`DELETE /v1/convai/tags/{tag_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tag-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations tags get`

Get a conversation tag by ID.

`GET /v1/convai/tags/{tag_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tag-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations tags list`

List conversation tags for the workspace, ordered by most recently created first.

`GET /v1/convai/tags`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-size` | `integer` | No | How many conversation tags to return. Can not exceed 100. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations tags unassign`

Remove a single conversation tag from a conversation.

`DELETE /v1/convai/conversations/{conversation_id}/tags/{tag_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--conversation-id` | `string` | Yes |  |
| `--tag-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents conversations tags update`

Update a conversation tag's title and/or description. Restricted to the tag owner or a workspace admin.

`PATCH /v1/convai/tags/{tag_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tag-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents conversations topics`

#### `elevenlabs agents conversations topics get`

Returns the latest topic discovery run results for a given agent.

`GET /v1/convai/agents/{agent_id}/topics`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | ID of the agent |
| `--page-size` | `string` | No | Number of top-level topic groups to return. |
| `--sort-by` | `TopicSortBy` | No | Column to rank topics by. Use conversations for volume, sentiment with sort_direction=asc for the most negative topics, and frustration with sort_direction=desc for the most frustrated ones. Topics with no score are always ranked last. |
| `--sort-direction` | `SortDirection` | No | Direction to sort topics. |
| `--from-unix-secs` | `string` | No | Start of the window to view topics for. When set with to_unix_secs, the completed daily topic-discovery runs in the range are aggregated together, so the window scopes the metrics as well as the topic set. Floored to the start of its UTC day because runs cover whole UTC days; aggregated_run_count reports how many runs were summed. Omit both bounds to get the single latest run. |
| `--to-unix-secs` | `string` | No | End of the window to view topics for. |
| `--include-evaluation-criteria` | `boolean` | No | Include the per-criteria evaluation breakdown on each topic's metrics. Pass false to drop it: it dominates the payload and the weighted success_rate is returned either way. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents dashboard settings`

#### `elevenlabs agents dashboard settings get`

Retrieve Convai dashboard settings for the workspace

`GET /v1/convai/settings/dashboard`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents dashboard settings update`

Update Convai dashboard settings for the workspace

`PATCH /v1/convai/settings/dashboard`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents deployments`

#### `elevenlabs agents deployments create`

Create a new deployment for an agent

`POST /v1/convai/agents/{agent_id}/deployments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents drafts`

#### `elevenlabs agents drafts create`

Create a new draft for an agent

`POST /v1/convai/agents/{agent_id}/drafts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--branch-id` | `string` | Yes | The ID of the agent branch to use |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents drafts delete`

Delete a draft for an agent

`DELETE /v1/convai/agents/{agent_id}/drafts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--branch-id` | `string` | Yes | The ID of the agent branch to use |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents exotel`

#### `elevenlabs agents exotel outbound-call`

Handle an outbound call via Exotel Connect API

`POST /v1/convai/exotel/outbound-call`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents knowledge-base`

#### `elevenlabs agents knowledge-base get-or-create-rag-indexes`

Retrieves and/or creates RAG indexes for multiple knowledge base documents in a single request. Maximum 100 items per request.

`POST /v1/convai/knowledge-base/rag-index`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base list`

Get a list of available knowledge base documents

`GET /v1/convai/knowledge-base`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-size` | `integer` | No | How many documents to return at maximum. Can not exceed 100, defaults to 30. |
| `--search` | `string` | No | If specified, the endpoint returns only such knowledge base documents whose names start with this string. |
| `--show-only-owned-documents` | `boolean` | No | If set to true, the endpoint will return only documents owned by you (and not shared from somebody else). Deprecated: use created_by_user_id instead. |
| `--created-by-user-id` | `string` | No | Filter documents by creator user ID. When set, only documents created by this user are returned. Takes precedence over show_only_owned_documents. Use '@me' to refer to the authenticated user. |
| `--types` | `string` | No | If present, the endpoint will return only documents of the given types. |
| `--parent-folder-id` | `string` | No | If set, the endpoint will return only documents that are direct children of the given folder. |
| `--ancestor-folder-id` | `string` | No | If set, the endpoint will return only documents that are descendants of the given folder. |
| `--folders-first` | `boolean` | No | Whether folders should be returned first in the list of documents. |
| `--sort-direction` | `SortDirection` | No | The direction to sort the results |
| `--sort-by` | `string` | No | The field to sort the results by |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents knowledge-base search`

Fuzzy text search over knowledge base document content

`GET /v1/convai/knowledge-base/search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--query` | `string` | Yes | The search query text |
| `--page-size` | `integer` | No | How many documents to return at maximum. Can not exceed 100, defaults to 30. |
| `--types` | `string` | No | If present, the endpoint will return only documents of the given types. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents knowledge-base size`

Returns the number of pages in the agent's knowledge base.

`GET /v1/convai/agent/{agent_id}/knowledge-base/size`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents knowledge-base crawl-jobs`

#### `elevenlabs agents knowledge-base crawl-jobs cancel`

Cancel a crawl job and delete all associated documents and folders.

`POST /v1/convai/knowledge-base/crawl/{crawl_job_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--crawl-job-id` | `string` | Yes | The id of the crawl job to retrieve |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents knowledge-base crawl-jobs create`

Create a crawl job to crawl the given URL with specified depth and page limits.

`POST /v1/convai/knowledge-base/crawl`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base crawl-jobs get`

Get details about a specific crawl job including status and progress.

`GET /v1/convai/knowledge-base/crawl/{crawl_job_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--crawl-job-id` | `string` | Yes | The id of the crawl job to retrieve |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents knowledge-base crawl-jobs list`

Get a list of ongoing and recent crawl jobs for the user.

`GET /v1/convai/knowledge-base/crawl`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--include-job-ids` | `string` | No | Ids of additional crawl jobs to retrieve |
| `--page-size` | `integer` | No | How many documents to return at maximum. Can not exceed 100, defaults to 30. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents knowledge-base document`

#### `elevenlabs agents knowledge-base document compute-rag-index`

In case the document is not RAG indexed, it triggers rag indexing task, otherwise it just returns the current status.

`POST /v1/convai/knowledge-base/{documentation_id}/rag-index`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base document refresh`

Manually refresh a URL document by re-fetching its content from the source URL.

`POST /v1/convai/knowledge-base/{documentation_id}/refresh`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents knowledge-base document update-file`

Update the source file of a file document. The document name, content, and metadata are updated to reflect the new file. Any manual content edits will be overwritten.

`PATCH /v1/convai/knowledge-base/{documentation_id}/update-file`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents knowledge-base documents`

#### `elevenlabs agents knowledge-base documents bulk-delete`

Delete multiple documents or folders from the knowledge base. Each id succeeds or fails independently.

`POST /v1/convai/knowledge-base/bulk-delete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base documents bulk-move`

Moves multiple entities from one folder to another.

`POST /v1/convai/knowledge-base/bulk-move`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base documents create-folder`

Create a folder used for grouping documents together.

`POST /v1/convai/knowledge-base/folder`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base documents create-from-file`

Create a knowledge base document generated form the uploaded file.

`POST /v1/convai/knowledge-base/file`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base documents create-from-text`

Create a knowledge base document containing the provided text.

`POST /v1/convai/knowledge-base/text`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base documents create-from-url`

Create a knowledge base document generated by scraping the given webpage.

`POST /v1/convai/knowledge-base/url`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base documents delete`

Delete a document or folder from the knowledge base.

`DELETE /v1/convai/knowledge-base/{documentation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--force` | `boolean` | No | If set to true, the document or folder will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents. For non-empty folders, this will also delete all child documents and folders. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents knowledge-base documents get`

Get details about a specific documentation making up the agent's knowledge base

`GET /v1/convai/knowledge-base/{documentation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--agent-id` | `string` | No |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents knowledge-base documents get-agents`

Get a list of agents depending on this knowledge base document

`GET /v1/convai/knowledge-base/{documentation_id}/dependent-agents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--dependent-type` | `KnowledgeBaseDependentType` | No | Type of dependent agents to return. |
| `--page-size` | `integer` | No | How many documents to return at maximum. Can not exceed 100, defaults to 30. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents knowledge-base documents get-bulk-agents`

Get a list of agents depending on any of the given knowledge base documents.

`POST /v1/convai/knowledge-base/dependent-agents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dependent-type` | `KnowledgeBaseDependentType` | No | Type of dependent agents to return. |
| `--page-size` | `integer` | No | How many documents to return at maximum. Can not exceed 100, defaults to 30. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base documents get-content`

Get the entire content of a document from the knowledge base

`GET /v1/convai/knowledge-base/{documentation_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents knowledge-base documents get-source-file-url`

Get a signed URL to download the original source file of a file-type document from the knowledge base

`GET /v1/convai/knowledge-base/{documentation_id}/source-file-url`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents knowledge-base documents move`

Moves the entity from one folder to another.

`POST /v1/convai/knowledge-base/{document_id}/move`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--document-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents knowledge-base documents update`

Update the name and/or content of a document.

`PATCH /v1/convai/knowledge-base/{documentation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents knowledge-base documents chunk`

#### `elevenlabs agents knowledge-base documents chunk get`

Get details about a specific documentation part used by RAG.

`GET /v1/convai/knowledge-base/{documentation_id}/chunk/{chunk_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--chunk-id` | `string` | Yes | The id of a document RAG chunk from the knowledge base. |
| `--embedding-model` | `string` | No | The embedding model used to retrieve the chunk. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents knowledge-base documents chunks`

#### `elevenlabs agents knowledge-base documents chunks list`

Get all RAG chunks for a specific knowledge base document.

`GET /v1/convai/knowledge-base/{documentation_id}/chunks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--documentation-id` | `string` | Yes | The id of a document from the knowledge base. This is returned on document addition. |
| `--embedding-model` | `EmbeddingModelEnum` | Yes | The embedding model used to retrieve the chunk. |
| `--page-size` | `integer` | No | How many documents to return at maximum. Can not exceed 100, defaults to 30. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents knowledge-base documents summaries`

#### `elevenlabs agents knowledge-base documents summaries get`

Gets multiple knowledge base document summaries by their IDs.

`GET /v1/convai/knowledge-base/summaries`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--document-ids` | `string[]` | Yes | The ids of knowledge base documents. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents link`

#### `elevenlabs agents link get`

Get the current link used to share the agent with others

`GET /v1/convai/agents/{agent_id}/link`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents llm`

#### `elevenlabs agents llm list`

Returns a list of available LLM models that can be used with agents, including their capabilities and any deprecation status. The response is filtered based on the data residency of the deployment and any compliance requirements (e.g. HIPAA) of the workspace subscription.

`GET /v1/convai/llm/list`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents llm-usage`

#### `elevenlabs agents llm-usage calculate`

Returns a list of LLM models and the expected cost for using them based on the provided values.

`POST /v1/convai/llm-usage/calculate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents mcp-servers`

#### `elevenlabs agents mcp-servers create`

Create a new MCP server configuration in the workspace.

`POST /v1/convai/mcp-servers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents mcp-servers delete`

Delete a specific MCP server configuration from the workspace.

`DELETE /v1/convai/mcp-servers/{mcp_server_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents mcp-servers get`

Retrieve a specific MCP server configuration from the workspace.

`GET /v1/convai/mcp-servers/{mcp_server_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents mcp-servers list`

Retrieve all MCP server configurations available in the workspace.

`GET /v1/convai/mcp-servers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents mcp-servers update`

Update the configuration settings for an MCP server.

`PATCH /v1/convai/mcp-servers/{mcp_server_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents mcp-servers approval-policy`

#### `elevenlabs agents mcp-servers approval-policy update` `[DEPRECATED]`

Update the approval policy configuration for an MCP server. DEPRECATED: Use PATCH /mcp-servers/{id} endpoint instead.

`PATCH /v1/convai/mcp-servers/{mcp_server_id}/approval-policy`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents mcp-servers tool-approvals`

#### `elevenlabs agents mcp-servers tool-approvals create`

Add approval for a specific MCP tool when using per-tool approval mode.

`POST /v1/convai/mcp-servers/{mcp_server_id}/tool-approvals`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents mcp-servers tool-approvals delete`

Remove approval for a specific MCP tool when using per-tool approval mode.

`DELETE /v1/convai/mcp-servers/{mcp_server_id}/tool-approvals/{tool_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--tool-name` | `string` | Yes | Name of the MCP tool to remove approval for. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents mcp-servers tool-configs`

#### `elevenlabs agents mcp-servers tool-configs create`

Create configuration overrides for a specific MCP tool.

`POST /v1/convai/mcp-servers/{mcp_server_id}/tool-configs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--environment` | `string` | No | Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents mcp-servers tool-configs delete`

Remove configuration overrides for a specific MCP tool.

`DELETE /v1/convai/mcp-servers/{mcp_server_id}/tool-configs/{tool_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--tool-name` | `string` | Yes | Name of the MCP tool to remove config overrides for. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents mcp-servers tool-configs get`

Retrieve configuration overrides for a specific MCP tool.

`GET /v1/convai/mcp-servers/{mcp_server_id}/tool-configs/{tool_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--tool-name` | `string` | Yes | Name of the MCP tool to retrieve config overrides for. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents mcp-servers tool-configs update`

Update configuration overrides for a specific MCP tool.

`PATCH /v1/convai/mcp-servers/{mcp_server_id}/tool-configs/{tool_name}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--tool-name` | `string` | Yes | Name of the MCP tool to update config overrides for. |
| `--environment` | `string` | No | Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents mcp-servers tools`

#### `elevenlabs agents mcp-servers tools list`

Retrieve all tools available for a specific MCP server configuration.

`GET /v1/convai/mcp-servers/{mcp_server_id}/tools`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--mcp-server-id` | `string` | Yes | ID of the MCP Server. |
| `--environment` | `string` | No | Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents phone-numbers`

#### `elevenlabs agents phone-numbers create`

Import Phone Number from provider configuration (Twilio, Exotel, or SIP trunk)

`POST /v1/convai/phone-numbers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents phone-numbers delete`

Delete Phone Number by ID

`DELETE /v1/convai/phone-numbers/{phone_number_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--phone-number-id` | `string` | Yes | The phone number ID. This is returned when a phone number is imported. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents phone-numbers get`

Retrieve Phone Number details by ID

`GET /v1/convai/phone-numbers/{phone_number_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--phone-number-id` | `string` | Yes | The phone number ID. This is returned when a phone number is imported. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents phone-numbers get-sip-messages`

Get SIP messages for a phone number

`GET /v1/convai/phone-numbers/{phone_number_id}/sip-messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--phone-number-id` | `string` | Yes | The phone number ID. This is returned when a phone number is imported. |
| `--page-size` | `integer` | No |  |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents phone-numbers list`

Retrieve all Phone Numbers

`GET /v1/convai/phone-numbers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--provider` | `string` | No | Filter by telephony provider |
| `--agent-id` | `string` | No | Filter by assigned agent ID |
| `--branch-id` | `string` | No | Filter by assigned branch ID |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents phone-numbers update`

Update assigned agent of a phone number

`PATCH /v1/convai/phone-numbers/{phone_number_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--phone-number-id` | `string` | Yes | The phone number ID. This is returned when a phone number is imported. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents procedures`

#### `elevenlabs agents procedures compile`

Compile procedure drafts into a workflow.

`POST /v1/convai/agents/{agent_id}/branches/{branch_id}/procedures/compile`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent ID to get the procedure draft from |
| `--branch-id` | `string` | Yes | Branch ID to get the procedure draft from |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents procedures create`

Create a new procedure for the agent on a branch.

`POST /v1/convai/agents/{agent_id}/branches/{branch_id}/procedures`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent ID to get the procedure draft from |
| `--branch-id` | `string` | Yes | Branch ID to get the procedure draft from |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents procedures get`

Retrieve a procedure at a specific version or the current branch HEAD.

`GET /v1/convai/agents/{agent_id}/branches/{branch_id}/procedures/{procedure_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent ID to get the procedure draft from |
| `--branch-id` | `string` | Yes | Branch ID to get the procedure draft from |
| `--procedure-id` | `string` | Yes | The procedure ID |
| `--version-id` | `string` | No | The version ID to retrieve. If omitted, returns the version at branch HEAD. |
| `--agent-version-id` | `string` | No | The agent version ID to retrieve the procedure for. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents procedures list`

List the agent's procedures on a branch with their procedure_id, version_id, name, type, trigger, and has_draft. has_draft is true when a procedure has unpublished draft changes on this branch; its name/type/trigger then reflect that draft. Does not return procedure content -- use Get Procedure to read a procedure's body.

`GET /v1/convai/agents/{agent_id}/branches/{branch_id}/procedures`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent ID to get the procedure draft from |
| `--branch-id` | `string` | Yes | Branch ID to get the procedure draft from |
| `--agent-version-id` | `string` | No | The agent version ID to retrieve the procedure for. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents procedures remove`

Remove a procedure from the agent's draft working set. Removing a folder cascades to its entire subtree, rejected if any procedure outside the subtree hands off into it.

`DELETE /v1/convai/agents/{agent_id}/branches/{branch_id}/procedures/{procedure_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent ID to get the procedure draft from |
| `--branch-id` | `string` | Yes | Branch ID to get the procedure draft from |
| `--procedure-id` | `string` | Yes | The procedure ID |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents procedures drafts`

#### `elevenlabs agents procedures drafts delete`

Delete user's draft for a procedure, resetting to the committed version

`DELETE /v1/convai/agents/{agent_id}/branches/{branch_id}/procedures/{procedure_id}/draft`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent ID to get the procedure draft from |
| `--branch-id` | `string` | Yes | Branch ID to get the procedure draft from |
| `--procedure-id` | `string` | Yes | The procedure ID |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents procedures drafts get`

Get user's draft for a procedure

`GET /v1/convai/agents/{agent_id}/branches/{branch_id}/procedures/{procedure_id}/draft`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent ID to get the procedure draft from |
| `--branch-id` | `string` | Yes | Branch ID to get the procedure draft from |
| `--procedure-id` | `string` | Yes | The procedure ID |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents procedures drafts update`

Create or update user's draft for a procedure

`PATCH /v1/convai/agents/{agent_id}/branches/{branch_id}/procedures/{procedure_id}/draft`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | Agent ID to get the procedure draft from |
| `--branch-id` | `string` | Yes | Branch ID to get the procedure draft from |
| `--procedure-id` | `string` | Yes | The procedure ID |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents secrets`

#### `elevenlabs agents secrets create`

Create a new secret for the workspace

`POST /v1/convai/secrets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents secrets delete`

Delete a workspace secret if it's not in use

`DELETE /v1/convai/secrets/{secret_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--secret-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents secrets get`

Get a workspace secret by ID

`GET /v1/convai/secrets/{secret_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--secret-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents secrets get-dependencies`

Get paginated list of resources that depend on a specific secret, filtered by resource type.

`GET /v1/convai/secrets/{secret_id}/dependencies/{resource_type}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--secret-id` | `string` | Yes |  |
| `--resource-type` | `SecretDependencyResourceType` | Yes |  |
| `--page-size` | `integer` | No | How many dependency items to return per page. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents secrets list`

Get all workspace secrets for the user

`GET /v1/convai/secrets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-size` | `string` | No | How many documents to return at maximum. Can not exceed 100. If not provided, returns all secrets. |
| `--dependency-limit` | `string` | No | Maximum number of dependent resources (tools, agents, phone numbers) to return per secret. Can not exceed 100. |
| `--search` | `string` | No | If specified, returns only secrets whose names start with this string. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents secrets update`

Update an existing secret for the workspace

`PATCH /v1/convai/secrets/{secret_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--secret-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents settings`

#### `elevenlabs agents settings get`

Retrieve Convai settings for the workspace

`GET /v1/convai/settings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents settings update`

Update Convai settings for the workspace

`PATCH /v1/convai/settings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents sip-trunk`

#### `elevenlabs agents sip-trunk outbound-call`

Handle an outbound call via SIP trunk

`POST /v1/convai/sip-trunk/outbound-call`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents summaries`

#### `elevenlabs agents summaries get`

Returns summaries for the specified agents.

`GET /v1/convai/agents/summaries`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-ids` | `string[]` | Yes | List of agent IDs to fetch summaries for |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents tests`

#### `elevenlabs agents tests create`

Creates a new agent response test.

`POST /v1/convai/agent-testing/create`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents tests delete`

Deletes an agent response test by ID.

`DELETE /v1/convai/agent-testing/{test_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--test-id` | `string` | Yes | The id of a chat response test. This is returned on test creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tests get`

Gets an agent response test by ID.

`GET /v1/convai/agent-testing/{test_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--test-id` | `string` | Yes | The id of a chat response test. This is returned on test creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tests list`

Lists all agent response tests with pagination support and optional search filtering.

`GET /v1/convai/agent-testing`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--page-size` | `integer` | No | How many Tests to return at maximum. Can not exceed 100, defaults to 30. |
| `--search` | `string` | No | Search query to filter tests by name. |
| `--parent-folder-id` | `string` | No | Filter by parent folder ID. Use 'root' to get items in the root folder. |
| `--types` | `string` | No | If present, the endpoint will return only tests/folders of the given types. |
| `--include-folders` | `string` | No | Deprecated. Use the `types` query param and include `folder` instead. |
| `--sort-mode` | `default | folders_first` | No | Sort mode for listing tests. Use 'folders_first' to place folders before tests. |
| `--sharing-mode` | `TestSharingMode` | No | Filter test visibility. Use `shared_with_me` to return only tests/folders shared with the current user that they did not create. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tests move`

Moves multiple tests or folders from one folder to another.

`POST /v1/convai/agent-testing/bulk-move`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents tests summaries`

Gets agent response test summaries for the requested test IDs.

`POST /v1/convai/agent-testing/summaries`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents tests update`

Updates an agent response test by ID.

`PUT /v1/convai/agent-testing/{test_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--test-id` | `string` | Yes | The id of a chat response test. This is returned on test creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents tests folders`

#### `elevenlabs agents tests folders create`

Creates a folder for organizing agent tests.

`POST /v1/convai/agent-testing/folders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents tests folders delete`

Deletes an agent test folder by ID. Use force=true to delete a non-empty folder and all its contents.

`DELETE /v1/convai/agent-testing/folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--folder-id` | `string` | Yes | The folder ID. |
| `--force` | `boolean` | No | Force delete. Required for deleting non-empty folders. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tests folders get`

Gets an agent test folder by ID, including its folder path.

`GET /v1/convai/agent-testing/folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--folder-id` | `string` | Yes | The folder ID. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tests folders update`

Updates an agent test folder. Currently only supports updating the folder name.

`PATCH /v1/convai/agent-testing/folders/{folder_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--folder-id` | `string` | Yes | The folder ID. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents tests invocations`

#### `elevenlabs agents tests invocations get`

Gets a test invocation by ID.

`GET /v1/convai/test-invocations/{test_invocation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--test-invocation-id` | `string` | Yes | The id of a test invocation. This is returned when tests are run. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tests invocations list`

Lists all test invocations with pagination support and optional search filtering.

`GET /v1/convai/test-invocations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | No | Filter by agent ID |
| `--page-size` | `integer` | No | How many Tests to return at maximum. Can not exceed 100, defaults to 30. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tests invocations resubmit`

Resubmits specific test runs from a test invocation.

`POST /v1/convai/test-invocations/{test_invocation_id}/resubmit`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--test-invocation-id` | `string` | Yes | The id of a test invocation. This is returned when tests are run. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents tools`

#### `elevenlabs agents tools create`

Add a new tool to the available tools in the workspace.

`POST /v1/convai/tools`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents tools delete`

Delete tool from the workspace.

`DELETE /v1/convai/tools/{tool_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-id` | `string` | Yes | ID of the requested tool. |
| `--force` | `boolean` | No | If set to true, the tool will be deleted regardless of whether it is used by any agents and it will be removed from the dependent agents and branches. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tools get`

Get tool that is available in the workspace.

`GET /v1/convai/tools/{tool_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-id` | `string` | Yes | ID of the requested tool. |
| `--environment` | `string` | No | Environment whose values are used when the MCP server URL, headers, or auth connection reference environment variables. Mirrors the environment a conversation would run in; defaults to production. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tools get-dependent-agents`

Get a list of agents depending on this tool

`GET /v1/convai/tools/{tool_id}/dependent-agents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-id` | `string` | Yes | ID of the requested tool. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--page-size` | `integer` | No | How many documents to return at maximum. Can not exceed 100, defaults to 30. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tools list`

Get all available tools in the workspace.

`GET /v1/convai/tools`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--search` | `string` | No | If specified, the endpoint returns only tools whose names start with this string. |
| `--page-size` | `string` | No | How many documents to return at maximum. Can not exceed 100, defaults to 30. |
| `--show-only-owned-documents` | `boolean` | No | If set to true, the endpoint will return only tools owned by you (and not shared from somebody else). Deprecated: use created_by_user_id instead. |
| `--created-by-user-id` | `string` | No | Filter tools by creator user ID. When set, only tools created by this user are returned. Takes precedence over show_only_owned_documents. Use '@me' to refer to the authenticated user. |
| `--types` | `string` | No | If present, the endpoint will return only tools of the given types. |
| `--sort-direction` | `SortDirection` | No | The direction to sort the results |
| `--sort-by` | `string` | No | The field to sort the results by |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents tools update`

Update tool that is available in the workspace.

`PATCH /v1/convai/tools/{tool_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-id` | `string` | Yes | ID of the requested tool. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents tools executions`

#### `elevenlabs agents tools executions get`

Get paginated list of tool executions for a specific tool.

`GET /v1/convai/tools/{tool_id}/executions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--tool-id` | `string` | Yes | ID of the requested tool. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--page-size` | `integer` | No | How many documents to return at maximum. Can not exceed 100, defaults to 30. |
| `--is-error` | `string` | No | Filter by error status. If not provided, returns all executions. |
| `--agent-id` | `string` | No | Filter by agent ID. |
| `--branch-id` | `string` | No | Filter by agent branch ID. |
| `--start-time` | `string` | No | Filter executions from this Unix timestamp (inclusive). |
| `--end-time` | `string` | No | Filter executions until this Unix timestamp (inclusive). |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents triage-tickets`

#### `elevenlabs agents triage-tickets add-comment`

Append a comment discussing how to resolve the ticket. Requires viewer access to the ticket's agent.

`POST /v1/convai/triage-tickets/{agentqa_ticket_id}/comments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agentqa-ticket-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents triage-tickets add-turn-comment`

Append a turn-level comment to a ticket. Requires viewer access to the ticket's agent.

`POST /v1/convai/triage-tickets/{agentqa_ticket_id}/turn-comments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agentqa-ticket-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents triage-tickets create`

Raise a ticket about an agent's performance on a conversation, for triage with Architect. Provide an overall comment and/or turn-level comments describing what went wrong.

`POST /v1/convai/triage-tickets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents triage-tickets create-manual`

Manually raise a follow-up ticket against an agent, not tied to any conversation (for example a task like 'add the KB about X'). The comment is shown as the ticket title. Requires viewer access to the agent.

`POST /v1/convai/agents/{agent_id}/triage-tickets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents triage-tickets delete`

Delete an agent conversation ticket. Restricted to the ticket creator or a workspace admin.

`DELETE /v1/convai/triage-tickets/{agentqa_ticket_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agentqa-ticket-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents triage-tickets get`

Get an agent conversation ticket by ID.

`GET /v1/convai/triage-tickets/{agentqa_ticket_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agentqa-ticket-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents triage-tickets list`

List an agent's conversation triage tickets, ordered by most recently created first. These are tickets about the agent's own performance on a conversation (for triage with Architect), not tickets an agent opens for end users.

`GET /v1/convai/agents/{agent_id}/triage-tickets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes |  |
| `--page-size` | `integer` | No | How many agent conversation tickets to return. Can not exceed 100. |
| `--conversation-id` | `string` | No | Filter tickets by conversation id. |
| `--status` | `string` | No | Filter tickets by status. |
| `--sources` | `string` | No | Filter tickets by how they were raised (qa, agent, manual). Repeat the parameter to filter by multiple sources. |
| `--owner-user-id` | `string` | No | Filter tickets by creator. Use 'agent' for agent-raised tickets. |
| `--assignee-user-id` | `string` | No | Filter tickets by assignee. Use 'unassigned' for tickets with no assignee. |
| `--issue-type` | `string` | No | Filter clusters by issue type. |
| `--label` | `string` | No | Filter tickets by an exact label. |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents triage-tickets list-assignable-users`

All non-service-account workspace members, each flagged with whether they currently have at least viewer access to the agent. Members without access are included (not filtered out) so the UI can offer them as an assignee and prompt to grant access first.

`GET /v1/convai/agents/{agent_id}/triage-tickets/assignable-users`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents triage-tickets update`

Update a ticket's comment, status, and/or assignee. Requires editor access to the ticket's agent.

`PATCH /v1/convai/triage-tickets/{agentqa_ticket_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agentqa-ticket-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents twilio`

#### `elevenlabs agents twilio outbound-call`

Handle an outbound call via Twilio

`POST /v1/convai/twilio/outbound-call`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents twilio register-call`

Register a Twilio call and return TwiML to connect the call

`POST /v1/convai/twilio/register-call`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents users`

#### `elevenlabs agents users list`

Get distinct users from conversations with pagination.

`GET /v1/convai/users`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | No | Agent id (agent_…) or speech engine external id (seng_), resolved to the same underlying resource. |
| `--branch-id` | `string` | No | Filter conversations by branch ID. |
| `--call-start-before-unix` | `string` | No | Unix timestamp (in seconds) to filter conversations up to this start date. |
| `--call-start-after-unix` | `string` | No | Unix timestamp (in seconds) to filter conversations after to this start date. |
| `--search` | `string` | No | Search/filter by user ID (exact match). |
| `--page-size` | `integer` | No | How many users to return at maximum. Defaults to 30. |
| `--sort-by` | `UsersSortBy` | No | The field to sort the results by. Defaults to last_contact_unix_secs. |
| `--sort-direction` | `SortDirection` | No | The direction to sort the results |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents versions`

#### `elevenlabs agents versions get`

Get metadata for a specific agent version

`GET /v1/convai/agents/{agent_id}/versions/{version_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--version-id` | `string` | Yes | Unique identifier for the version. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents whatsapp`

#### `elevenlabs agents whatsapp outbound-call`

Make an outbound call via WhatsApp

`POST /v1/convai/whatsapp/outbound-call`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs agents whatsapp outbound-message`

Send an outbound message via WhatsApp

`POST /v1/convai/whatsapp/outbound-message`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents whatsapp-accounts`

#### `elevenlabs agents whatsapp-accounts delete`

Delete a WhatsApp account

`DELETE /v1/convai/whatsapp-accounts/{phone_number_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--phone-number-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents whatsapp-accounts get`

Get a WhatsApp account

`GET /v1/convai/whatsapp-accounts/{phone_number_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--phone-number-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents whatsapp-accounts list`

List all WhatsApp accounts

`GET /v1/convai/whatsapp-accounts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | No | Filter by assigned agent ID |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs agents whatsapp-accounts update`

Update a WhatsApp account

`PATCH /v1/convai/whatsapp-accounts/{phone_number_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--phone-number-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs agents widget`

#### `elevenlabs agents widget get`

Retrieve the widget configuration for an agent

`GET /v1/convai/agents/{agent_id}/widget`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--conversation-signature` | `string` | No | An expiring token that enables a websocket conversation to start. These can be generated for an agent using the /v1/convai/conversation/get_signed_url endpoint |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs agents widget avatar`

#### `elevenlabs agents widget avatar create`

Sets the avatar for an agent displayed in the widget

`POST /v1/convai/agents/{agent_id}/avatar`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--agent-id` | `string` | Yes | The id of an agent. This is returned on agent creation. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs assets`

#### `elevenlabs assets create`

Upload a new asset.

`POST /v1/assets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs assets delete`

Delete an asset by ID.

`DELETE /v1/assets/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | ID of the asset. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs assets get`

Retrieve a single asset by ID.

`GET /v1/assets/{asset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--asset-id` | `string` | Yes | ID of the asset. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs assets list`

List assets in the workspace, most recently created first.

`GET /v1/assets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-size` | `integer` | No | Number of assets to return. |
| `--cursor` | `string` | No | Token from a previous response's `next_cursor`. Omit to fetch the first page. |
| `--search` | `string` | No | Optional free-text search filter over asset names. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs audio-isolation`

#### `elevenlabs audio-isolation convert`

Removes background noise from audio.

`POST /v1/audio-isolation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs audio-isolation delete`

Deletes a specific audio isolation history item and the associated media files.

`DELETE /v1/audio-isolation/history/{history_item_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--history-item-id` | `string` | Yes | Identifier of the audio isolation history item. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs audio-isolation list`

Returns a list of all your audio isolation generations.

`GET /v1/audio-isolation/history`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-size` | `integer` | No | How many history items to return at maximum. Defaults to 100. |
| `--page` | `integer` | No | Page number for search pagination (1-based). Only used when search is provided. |
| `--search` | `string` | No | Optional search term used for filtering audio isolation history (title/text). |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs audio-isolation stream`

Removes background noise from audio.

`POST /v1/audio-isolation/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs audio-native`

#### `elevenlabs audio-native create`

Creates Audio Native enabled project, optionally starts conversion and returns project ID and embeddable HTML snippet.

`POST /v1/audio-native`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs audio-native get-settings`

Get player settings for the specific project.

`GET /v1/audio-native/{project_id}/settings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the Studio project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs audio-native update`

Updates content for the specific AudioNative Project.

`POST /v1/audio-native/{project_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs audio-native update-content-from-url`

Finds an AudioNative project matching the provided URL, extracts content from the URL, updates the project content, and queues it for conversion and auto-publishing.

`POST /v1/audio-native/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs dubbing`

#### `elevenlabs dubbing create`

Dubs a provided audio or video file into given language.

`POST /v1/dubbing`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing delete`

Deletes a dubbing project.

`DELETE /v1/dubbing/{dubbing_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing get`

Returns metadata about a dubbing project, including whether it's still in progress or not

`GET /v1/dubbing/{dubbing_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing list`

List the dubs you have access to.

`GET /v1/dubbing`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--page-size` | `integer` | No | How many dubs to return at maximum. Can not exceed 200, defaults to 100. |
| `--dubbing-status` | `dubbing | dubbed | failed` | No | What state the dub is currently in. |
| `--dubbing-statuses` | `string` | No | Filter by dubbing status. |
| `--dubbing-models` | `string` | No | Filter by dubbing model generation. |
| `--target-language-codes` | `string` | No | Filter by target language code. |
| `--creation-sources` | `string` | No | Filter by dubbing creation source. |
| `--filter-by-creator` | `personal | others | all` | No | Filters who created the resources being listed, whether it was the user running the request or someone else that shared the resource with them. |
| `--order-by` | `created_at | name` | No | The field to use for ordering results from this query. |
| `--order-direction` | `DESCENDING | ASCENDING` | No | The order direction to use for results from this query. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs dubbing audio`

#### `elevenlabs dubbing audio get`

Returns dub as a streamed MP3 or MP4 file. If this dub has been edited using Dubbing Studio you need to use the resource render endpoint as this endpoint only returns the original automatic dub result.

`GET /v1/dubbing/{dubbing_id}/audio/{language_code}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--language-code` | `string` | Yes | ID of the language. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs dubbing project`

#### `elevenlabs dubbing project create`

Create a dubbing project from an uploaded file or a source URL.

`POST /v1/dubbing/project`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing project delete`

Delete a project and its language targets.

`DELETE /v1/dubbing/project/{project_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project to delete. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing project get`

Full project detail, including its language target ids.

`GET /v1/dubbing/project/{project_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project to fetch. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing project list`

List the workspace's dubbing projects (cursor-paginated).

`GET /v1/dubbing/project`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--cursor` | `string` | No | Pagination cursor from a previous response's next_cursor. |
| `--page-size` | `integer` | No | Number of projects per page (max 100). |
| `--status` | `string` | No | Filter to projects in this status (preparing, ready, failed). |
| `--sort-direction` | `ASCENDING | DESCENDING` | No | Sort by creation time (default 'DESCENDING'). |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs dubbing project language`

#### `elevenlabs dubbing project language create`

Queue a language target for a project (starts once the project is ready).

`POST /v1/dubbing/project/{project_id}/language`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the parent dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing project language delete`

Delete a language target.

`DELETE /v1/dubbing/project/{project_id}/language/{language_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the parent dubbing project. |
| `--language-id` | `string` | Yes | Identifier of the language target to delete. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing project language get`

Full language-target detail.

`GET /v1/dubbing/project/{project_id}/language/{language_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the parent dubbing project. |
| `--language-id` | `string` | Yes | Identifier of the language target to fetch. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing project language list`

List a project's language targets (cursor-paginated).

`GET /v1/dubbing/project/{project_id}/language`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the parent dubbing project. |
| `--cursor` | `string` | No | Pagination cursor from a previous response's next_cursor. |
| `--page-size` | `integer` | No | Number of language targets per page (max 100). |
| `--status` | `string` | No | Filter to targets in this status (queued, processing, completed, stale, failed). |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs dubbing project language transcript`

#### `elevenlabs dubbing project language transcript get`

A language target's transcript: source segments with their translations.

`GET /v1/dubbing/project/{project_id}/language/{language_id}/transcript`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project. |
| `--language-id` | `string` | Yes | Identifier of the language target. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing project language transcript regenerate`

Enterprise only. Re-dub a target from its edited transcript, re-synthesizing only the edited regions (charged like a generation). Conflicts when the target has no edits to apply -- nothing is dispatched and nothing is charged.

`POST /v1/dubbing/project/{project_id}/language/{language_id}/transcript/regenerate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project. |
| `--language-id` | `string` | Yes | Identifier of the language target. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing project language transcript update-segment`

Enterprise only. Edit a segment's translation for a language target.

`PATCH /v1/dubbing/project/{project_id}/language/{language_id}/transcript/segment/{segment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project. |
| `--language-id` | `string` | Yes | Identifier of the language target. |
| `--segment-id` | `string` | Yes | Identifier of the segment to edit. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing project language transcript update-segments`

Enterprise only. Edit several segments' translations for a language target in one atomic request.

`PATCH /v1/dubbing/project/{project_id}/language/{language_id}/transcript/segments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project. |
| `--language-id` | `string` | Yes | Identifier of the language target. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs dubbing project transcript`

#### `elevenlabs dubbing project transcript create-segment`

Enterprise only. Add a new source segment to the transcript.

`POST /v1/dubbing/project/{project_id}/transcript/segment`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing project transcript delete-segment`

Enterprise only. Remove a source segment from the transcript.

`DELETE /v1/dubbing/project/{project_id}/transcript/segment/{segment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project. |
| `--segment-id` | `string` | Yes | Identifier of the segment to remove. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing project transcript get`

The project's source transcript, as editable segments.

`GET /v1/dubbing/project/{project_id}/transcript`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing project transcript update-segment`

Enterprise only. Edit a source segment's text, speaker, or timing.

`PATCH /v1/dubbing/project/{project_id}/transcript/segment/{segment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project. |
| `--segment-id` | `string` | Yes | Identifier of the segment to edit. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing project transcript update-segments`

Enterprise only. Edit several source segments' text, speaker, or timing in one atomic request.

`PATCH /v1/dubbing/project/{project_id}/transcript/segments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | Identifier of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs dubbing resource`

#### `elevenlabs dubbing resource dub` `[DEPRECATED]`

Regenerate the dubs for either the entire resource or the specified segments/languages. Will automatically transcribe and translate any missing transcriptions and translations.

`POST /v1/dubbing/resource/{dubbing_id}/dub`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing resource get` `[DEPRECATED]`

Given a dubbing ID generated from the '/v1/dubbing' endpoint with studio enabled, returns the dubbing resource.

`GET /v1/dubbing/resource/{dubbing_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing resource migrate-segments` `[DEPRECATED]`

Change the attribution of one or more segments to a different speaker.

`POST /v1/dubbing/resource/{dubbing_id}/migrate-segments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing resource render` `[DEPRECATED]`

Regenerate the output media for a language using the latest Studio state. Please ensure all segments have been dubbed before rendering, otherwise they will be omitted. Renders are generated asynchronously, and to check the status of all renders please use the 'Get Dubbing Resource' endpoint.

`POST /v1/dubbing/resource/{dubbing_id}/render/{language}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--language` | `string` | Yes | The target language code to render, eg. 'es'. To render the source track use 'original'. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing resource transcribe` `[DEPRECATED]`

Regenerate the transcriptions for the specified segments. Does not automatically regenerate translations or dubs.

`POST /v1/dubbing/resource/{dubbing_id}/transcribe`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing resource translate` `[DEPRECATED]`

Regenerate the translations for either the entire resource or the specified segments/languages. Will automatically transcribe missing transcriptions. Will not automatically regenerate the dubs.

`POST /v1/dubbing/resource/{dubbing_id}/translate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs dubbing resource language`

#### `elevenlabs dubbing resource language add` `[DEPRECATED]`

Adds the given ElevenLab Turbo V2/V2.5 language code to the resource. Does not automatically generate transcripts/translations/audio.

`POST /v1/dubbing/resource/{dubbing_id}/language`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs dubbing resource segment`

#### `elevenlabs dubbing resource segment delete` `[DEPRECATED]`

Deletes a single segment from the dubbing.

`DELETE /v1/dubbing/resource/{dubbing_id}/segment/{segment_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--segment-id` | `string` | Yes | ID of the segment |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing resource segment update` `[DEPRECATED]`

Modifies a single segment with new text and/or start/end times. Will update the values for only a specific language of a segment. Does not automatically regenerate the dub.

`PATCH /v1/dubbing/resource/{dubbing_id}/segment/{segment_id}/{language}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--segment-id` | `string` | Yes | ID of the segment |
| `--language` | `string` | Yes | ID of the language. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs dubbing resource speaker`

#### `elevenlabs dubbing resource speaker create` `[DEPRECATED]`

Creates a new speaker in a dubbing resource. The speaker is added to every available language and can optionally be associated with an ElevenLabs voice and voice settings.

`POST /v1/dubbing/resource/{dubbing_id}/speaker`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs dubbing resource speaker find-similar-voices` `[DEPRECATED]`

Fetch the top 10 similar voices to a speaker, including the voice IDs, names, descriptions, and, where possible, a sample audio recording.

`GET /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}/similar-voices`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--speaker-id` | `string` | Yes | ID of the speaker. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs dubbing resource speaker update` `[DEPRECATED]`

Amend the metadata associated with a speaker, such as their voice. Both voice cloning and using voices from the ElevenLabs library are supported.

`PATCH /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--speaker-id` | `string` | Yes | ID of the speaker. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs dubbing resource speaker segment`

#### `elevenlabs dubbing resource speaker segment create` `[DEPRECATED]`

Creates a new segment in dubbing resource with a start and end time for the speaker in every available language. Does not automatically generate transcripts/translations/audio.

`POST /v1/dubbing/resource/{dubbing_id}/speaker/{speaker_id}/segment`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--speaker-id` | `string` | Yes | ID of the speaker. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs dubbing transcript`

#### `elevenlabs dubbing transcript get-transcript-for-dub` `[DEPRECATED]`

Returns transcript for the dub as an SRT or WEBVTT file.

`GET /v1/dubbing/{dubbing_id}/transcript/{language_code}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--language-code` | `string` | Yes | ISO-693 language code to retrieve the transcript for. Use 'source' to fetch the transcript of the original media. |
| `--format-type` | `srt | webvtt | json` | No | Format to return transcript in. For subtitles use either 'srt' or 'webvtt', and for a full transcript use 'json'. The 'json' format is not yet supported for Dubbing Studio. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs dubbing transcripts`

#### `elevenlabs dubbing transcripts get`

Fetch the transcript for one of the languages in a dub.

`GET /v1/dubbing/{dubbing_id}/transcripts/{language_code}/format/{format_type}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dubbing-id` | `string` | Yes | ID of the dubbing project. |
| `--language-code` | `string` | Yes | ISO-693 language code to retrieve the transcript for. Use 'source' to fetch the transcript of the original media. |
| `--format-type` | `srt | webvtt | json` | Yes | Format to return transcript in. For subtitles use either 'srt' or 'webvtt', and for a full transcript use 'json'. The 'json' format is not yet supported for Dubbing Studio. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs environment-variables`

#### `elevenlabs environment-variables create`

Create a new environment variable for the workspace

`POST /v1/convai/environment-variables`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs environment-variables get`

Get a specific environment variable by ID

`GET /v1/convai/environment-variables/{env_var_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--env-var-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs environment-variables list`

List all environment variables for the workspace with optional filtering

`GET /v1/convai/environment-variables`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--cursor` | `string` | No | Pagination cursor from previous response |
| `--page-size` | `integer` | No | Number of items to return (1-100) |
| `--label` | `string` | No | Filter by exact label match |
| `--environment` | `string` | No | Filter to only return variables that have this environment. When specified, the values dict in the response will only contain this environment. |
| `--type` | `string` | No | Filter by variable type |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs environment-variables update`

Replace an environment variable's values. Use null to remove an environment (except production).

`PATCH /v1/convai/environment-variables/{env_var_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--env-var-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs flows image`

#### `elevenlabs flows image create`

Start an image generation with the selected model.

`POST /v1/flows/image`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs flows image get`

Retrieve the status of an image generation, and retrieve its output URL once completed.

`GET /v1/flows/image/{generation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--generation-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs flows image list`

List the image generations created through this API, newest first.

`GET /v1/flows/image`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--cursor` | `string` | No | Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page. |
| `--page-size` | `integer` | No | How many generations to return per page. |
| `--status` | `string` | No | Only return generations with this lifecycle status. |
| `--model-id` | `string` | No | Only return generations of this model. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs flows text-to-speech`

#### `elevenlabs flows text-to-speech create`

Start a speech generation with the selected model. Charged per character via text-to-speech billing. Use this over `/v1/text-to-speech` for the asynchronous generation lifecycle or for models not offered there; for direct, synchronous speech synthesis, prefer `/v1/text-to-speech`.

`POST /v1/flows/text-to-speech`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs flows text-to-speech get`

Retrieve the status of a speech generation, and retrieve its output URL once completed.

`GET /v1/flows/text-to-speech/{generation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--generation-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs flows text-to-speech list`

List the speech generations created through this API, newest first.

`GET /v1/flows/text-to-speech`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--cursor` | `string` | No | Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page. |
| `--page-size` | `integer` | No | How many generations to return per page. |
| `--status` | `string` | No | Only return generations with this lifecycle status. |
| `--model-id` | `string` | No | Only return generations of this model. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs flows video`

#### `elevenlabs flows video create`

Start a video generation with the selected model.

`POST /v1/flows/video`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs flows video get`

Retrieve the status of a video generation, and retrieve its output URL once completed.

`GET /v1/flows/video/{generation_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--generation-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs flows video list`

List the video generations created through this API, newest first.

`GET /v1/flows/video`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--cursor` | `string` | No | Pagination cursor: the `next_cursor` value of the previous page's response. Omit it for the first page. |
| `--page-size` | `integer` | No | How many generations to return per page. |
| `--status` | `string` | No | Only return generations with this lifecycle status. |
| `--model-id` | `string` | No | Only return generations of this model. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs forced-alignment`

#### `elevenlabs forced-alignment create`

Force align an audio file to text. Use this endpoint to get the timing information for each character and word in an audio file based on a provided text transcript.

`POST /v1/forced-alignment`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs history`

#### `elevenlabs history delete`

Delete a history item by its ID

`DELETE /v1/history/{history_item_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--history-item-id` | `string` | Yes | ID of the history item to be used. You can use the [Get generated items](/docs/api-reference/history/list) endpoint to retrieve a list of history items. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs history download`

Download one or more history items. If one history item ID is provided, we will return a single audio file. If more than one history item IDs are provided, we will provide the history items packed into a .zip file.

`POST /v1/history/download`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs history get`

Retrieves a history item.

`GET /v1/history/{history_item_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--history-item-id` | `string` | Yes | ID of the history item to be used. You can use the [Get generated items](/docs/api-reference/history/list) endpoint to retrieve a list of history items. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs history get-audio`

Returns the audio of an history item.

`GET /v1/history/{history_item_id}/audio`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--history-item-id` | `string` | Yes | ID of the history item to be used. You can use the [Get generated items](/docs/api-reference/history/list) endpoint to retrieve a list of history items. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs history list`

Returns a list of your generated audio (e.g. text to speech, speech to speech, Studio, dubbing). Music and SFX generations are not included and cannot currently be retrieved via the API.

`GET /v1/history`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-size` | `integer` | No | How many history items to return at maximum. Can not exceed 1000, defaults to 100. |
| `--start-after-history-item-id` | `string` | No | After which ID to start fetching, use this parameter to paginate across a large collection of history items. In case this parameter is not provided history items will be fetched starting from the most recently created one ordered descending by their creation date. |
| `--voice-id` | `string` | No | ID of the voice to be filtered for. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--model-id` | `string` | No | Search term used for filtering history items. If provided, source becomes required. |
| `--date-before-unix` | `string` | No | Unix timestamp to filter history items before this date (exclusive). |
| `--date-after-unix` | `string` | No | Unix timestamp to filter history items after this date (inclusive). |
| `--sort-direction` | `string` | No | Sort direction for the results. |
| `--search` | `string` | No | search term used for filtering |
| `--source` | `string` | No | Source of the generated history item |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs models`

#### `elevenlabs models list`

Gets a list of available models.

`GET /v1/models`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs music`

#### `elevenlabs music compose`

Compose a song from a prompt or a composition plan.

`POST /v1/music`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `auto | mp3_48000_128 | mp3_48000_192 | mp3_48000_240 | mp3_48000_320 | mp3_22050_32 | mp3_24000_48 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96 | opus_48000_128 | opus_48000_192` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. Use "auto" (the default) to let the API pick the best format for the selected model: mp3_44100_128 for v1 models and mp3_48000_192 for v2 models.  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs music compose-detailed`

Compose a song from a prompt or a composition plan.

`POST /v1/music/detailed`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `auto | mp3_48000_128 | mp3_48000_192 | mp3_48000_240 | mp3_48000_320 | mp3_22050_32 | mp3_24000_48 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96 | opus_48000_128 | opus_48000_192` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. Use "auto" (the default) to let the API pick the best format for the selected model: mp3_44100_128 for v1 models and mp3_48000_192 for v2 models.  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs music compose-detailed-stream`

Stream a song and its detailed metadata using Server-Sent Events (SSE).

`POST /v1/music/detailed/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `auto | mp3_48000_128 | mp3_48000_192 | mp3_48000_240 | mp3_48000_320 | mp3_22050_32 | mp3_24000_48 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96 | opus_48000_128 | opus_48000_192` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. Use "auto" (the default) to let the API pick the best format for the selected model: mp3_44100_128 for v1 models and mp3_48000_192 for v2 models.  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs music separate-stems`

Separate an audio file into individual stems. This endpoint might have high latency, depending on the length of the audio file.

`POST /v1/music/stem-separation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `AllowedOutputFormats` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs music stream`

Stream a composed song from a prompt or a composition plan.

`POST /v1/music/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `auto | mp3_48000_128 | mp3_48000_192 | mp3_48000_240 | mp3_48000_320 | mp3_22050_32 | mp3_24000_48 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96 | opus_48000_128 | opus_48000_192` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. Use "auto" (the default) to let the API pick the best format for the selected model: mp3_44100_128 for v1 models and mp3_48000_192 for v2 models.  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs music upload`

Upload a music file to be later used for inpainting. Price for uploading is the same as the one for song generation. All uploaded content gets inspected for copyright infringement. If copyrighted content is detected, half of the request cost is still charged.

`POST /v1/music/upload`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs music video-to-music`

Generate background music from one or more video files. Videos are combined in order. Optional description and style tags influence the generated music.

`POST /v1/music/video-to-music`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `AllowedOutputFormats` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs music composition-plan`

#### `elevenlabs music composition-plan create`

Create a composition plan for music generation. Usage of this endpoint does not cost any credits but is subject to rate limiting depending on your tier.

`POST /v1/music/plan`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs music finetunes`

#### `elevenlabs music finetunes create`

Create a new music finetune

`POST /v1/music/finetunes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs music finetunes delete`

Delete a music finetune

`DELETE /v1/music/finetunes/{finetune_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--finetune-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs music finetunes get`

Get a music finetune.

`GET /v1/music/finetunes/{finetune_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--finetune-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs music finetunes list`

List music finetunes accessible to you (your own, workspace-shared, and ElevenLabs-curated), with optional filtering, sorting, and cursor pagination.

`GET /v1/music/finetunes`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--cursor` | `string` | No | Used for fetching the next page. Cursor is returned in the response. |
| `--page-size` | `integer` | No | How many finetunes to return. Max 150, default 50. |
| `--visibility` | `string` | No | Filter by visibility. 'private' returns private finetunes; 'workspace' returns workspace-shared finetunes; 'public' returns public finetunes, which are currently ElevenLabs curated finetunes. Omit to return all accessible finetunes. |
| `--created-by` | `string` | No | Filter by creator. 'self' returns finetunes you created; 'workspace' returns finetunes created by workspace teammates; 'elevenlabs' returns ElevenLabs curated finetunes. Omit to return finetunes from all creators. |
| `--sort` | `created_at | name` | No | Sort by field (created_at or name) |
| `--sort-direction` | `asc | desc` | No | Sort direction (asc or desc) |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs music finetunes update`

Update a music finetune.

`PATCH /v1/music/finetunes/{finetune_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--finetune-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs productions orders`

#### `elevenlabs productions orders create`

Creates a new Productions order in the workspace. The order starts in the open state and can be configured with items before submission.

`POST /v1/productions/orders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs productions orders get`

Retrieves full details for a Productions order.

Quote and pricing information may not be available immediately; if you wish to see the quote before submission, you may need to poll the order details until it is ready.

`GET /v1/productions/orders/{order_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--order-id` | `OrderId` | Yes | The ID of the order. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs productions orders list`

Lists Productions orders in the workspace. Supports filtering by status and date range, with pagination.

`GET /v1/productions/orders`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-size` | `integer` | No | Maximum number of orders to return per page. |
| `--offset` | `integer` | No | Number of orders to skip for pagination. |
| `--status` | `string` | No | Filter orders by one or more statuses. |
| `--start-date` | `string` | No | Filter orders created on or after this date. |
| `--end-date` | `string` | No | Filter orders created on or before this date. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs productions orders submit`

Submits an open order for processing. The order must have at least one item. Once submitted, items can no longer be modified.

Upon submission, the workspace will be charged for the order. The quote is based on information extracted from the uploaded media, such as its duration. The quote may not be available immediately; if you wish to see the quote before submission, you may need to poll the order details until the quote is ready.

`POST /v1/productions/orders/{order_id}/submit`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--order-id` | `OrderId` | Yes | The ID of the order. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs productions orders update`

Updates an open order.

`PATCH /v1/productions/orders/{order_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--order-id` | `OrderId` | Yes | The ID of the order. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs productions orders deliverables`

#### `elevenlabs productions orders deliverables list`

Retrieves the delivered files for a completed order. Returns an empty list if the order is not yet completed.

`GET /v1/productions/orders/{order_id}/deliverables`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--order-id` | `OrderId` | Yes | The ID of the order. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs productions orders items`

#### `elevenlabs productions orders items remove`

Removes an order item from an open order.

`DELETE /v1/productions/orders/{order_id}/items/{item_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--order-id` | `OrderId` | Yes | The ID of the order. |
| `--item-id` | `ItemId` | Yes | The ID of the order item. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs productions orders items upsert`

Adds or updates an order item on an open order. Returns the item ID and the quoted price.

`POST /v1/productions/orders/{order_id}/items`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--order-id` | `OrderId` | Yes | The ID of the order. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs productions orders languages`

#### `elevenlabs productions orders languages list`

Returns the available languages for a given order item kind.

`GET /v1/productions/orders/languages/{order_item_kind}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--order-item-kind` | `OrderItemKind` | Yes | The kind of order item. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs productions orders media`

#### `elevenlabs productions orders media get`

Retrieves metadata and a time-limited download URL for a previously uploaded media file.

`GET /v1/productions/orders/{order_id}/media/{media_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--order-id` | `OrderId` | Yes | The ID of the order. |
| `--media-id` | `MediaId` | Yes | The ID of the media file. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs productions orders media register`

Registers a media file with an order, either by uploading it directly or by providing a URL to fetch it from. Exactly one of `media` or `media_url` must be provided. The registered media can then be referenced when adding order items.

`POST /v1/productions/orders/{order_id}/media`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--order-id` | `OrderId` | Yes | The ID of the order to which this media will be attached. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs pronunciation-dictionaries`

#### `elevenlabs pronunciation-dictionaries create-from-file`

Creates a new pronunciation dictionary from a lexicon .PLS file

`POST /v1/pronunciation-dictionaries/add-from-file`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs pronunciation-dictionaries create-from-rules`

Creates a new pronunciation dictionary from provided rules.

`POST /v1/pronunciation-dictionaries/add-from-rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs pronunciation-dictionaries download`

Get a PLS file with a pronunciation dictionary version rules

`GET /v1/pronunciation-dictionaries/{dictionary_id}/{version_id}/download`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--dictionary-id` | `string` | Yes | The id of the pronunciation dictionary |
| `--version-id` | `string` | Yes | The id of the pronunciation dictionary version |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs pronunciation-dictionaries get`

Get metadata for a pronunciation dictionary

`GET /v1/pronunciation-dictionaries/{pronunciation_dictionary_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pronunciation-dictionary-id` | `string` | Yes | The id of the pronunciation dictionary |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs pronunciation-dictionaries list`

Get a list of the pronunciation dictionaries you have access to and their metadata

`GET /v1/pronunciation-dictionaries`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--page-size` | `integer` | No | How many pronunciation dictionaries to return at maximum. Can not exceed 100, defaults to 30. |
| `--sort` | `string` | No | Which field to sort by, one of 'created_at_unix' or 'name'. |
| `--sort-direction` | `string` | No | Which direction to sort the voices in. 'ascending' or 'descending'. |
| `--include-archived` | `boolean` | No | Whether to include archived pronunciation dictionaries in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs pronunciation-dictionaries update`

Partially update the pronunciation dictionary without changing the version

`PATCH /v1/pronunciation-dictionaries/{pronunciation_dictionary_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pronunciation-dictionary-id` | `string` | Yes | The id of the pronunciation dictionary |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs pronunciation-dictionaries rules`

#### `elevenlabs pronunciation-dictionaries rules add`

Add rules to the pronunciation dictionary. If a rule with the same string_to_replace already exists, it will be replaced.

`POST /v1/pronunciation-dictionaries/{pronunciation_dictionary_id}/add-rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pronunciation-dictionary-id` | `string` | Yes | The id of the pronunciation dictionary |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs pronunciation-dictionaries rules remove`

Remove rules from the pronunciation dictionary

`POST /v1/pronunciation-dictionaries/{pronunciation_dictionary_id}/remove-rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pronunciation-dictionary-id` | `string` | Yes | The id of the pronunciation dictionary |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs pronunciation-dictionaries rules set`

Replaces all existing rules on the pronunciation dictionary with the provided ones.

`POST /v1/pronunciation-dictionaries/{pronunciation_dictionary_id}/set-rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pronunciation-dictionary-id` | `string` | Yes | The id of the pronunciation dictionary |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs samples`

#### `elevenlabs samples delete`

Removes a sample by its ID.

`DELETE /v1/voices/{voice_id}/samples/{sample_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--sample-id` | `string` | Yes | ID of the sample to be used. You can use the [Get voices](/docs/api-reference/voices/get) endpoint list all the available samples for a voice. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs service-accounts`

#### `elevenlabs service-accounts create`

Create a new service account in the workspace. By default, a workspace can have up to 20 service accounts. Enterprise customers may request an increase to this limit, up to 100.

`POST /v1/service-accounts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs service-accounts list`

List all service accounts in the workspace

`GET /v1/service-accounts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs service-accounts api-keys`

#### `elevenlabs service-accounts api-keys create`

Create a new API key for a service account

`POST /v1/service-accounts/{service_account_user_id}/api-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--service-account-user-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs service-accounts api-keys delete`

Delete an existing API key for a service account

`DELETE /v1/service-accounts/{service_account_user_id}/api-keys/{api_key_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--service-account-user-id` | `string` | Yes |  |
| `--api-key-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs service-accounts api-keys list`

Get all API keys for a service account

`GET /v1/service-accounts/{service_account_user_id}/api-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--service-account-user-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs service-accounts api-keys update`

Update an existing API key for a service account

`PATCH /v1/service-accounts/{service_account_user_id}/api-keys/{api_key_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--service-account-user-id` | `string` | Yes |  |
| `--api-key-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs speech-engine`

#### `elevenlabs speech-engine create`

Create a new Speech Engine resource

`POST /v1/speech-engine`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs speech-engine delete`

Delete a Speech Engine resource

`DELETE /v1/speech-engine/{speech_engine_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--speech-engine-id` | `string` | Yes | The speech engine ID (accepts seng_ or agent_ prefix) |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs speech-engine get`

Retrieve a Speech Engine resource

`GET /v1/speech-engine/{speech_engine_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--speech-engine-id` | `string` | Yes | The speech engine ID (accepts seng_ or agent_ prefix) |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs speech-engine list`

Returns a paginated list of Speech Engine resources.

`GET /v1/speech-engine`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-size` | `integer` | No | How many Speech Engines to return at maximum. Can not exceed 100, defaults to 30. |
| `--search` | `string` | No | Search term to filter Speech Engines by name |
| `--sort-direction` | `SortDirection` | No | The direction to sort the results |
| `--sort-by` | `string` | No | The field to sort the results by |
| `--cursor` | `string` | No | Used for fetching next page. Cursor is returned in the response. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs speech-engine update`

Update a Speech Engine resource (partial update)

`PATCH /v1/speech-engine/{speech_engine_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--speech-engine-id` | `string` | Yes | The speech engine ID (accepts seng_ or agent_ prefix) |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs speech-to-speech`

#### `elevenlabs speech-to-speech convert`

Transform audio from one voice to another. Maintain full control over emotion, timing and delivery.

`POST /v1/speech-to-speech/{voice_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers. |
| `--optimize-streaming-latency` | `string` | No | You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values:
0 - default mode (no latency optimizations)
1 - normal latency optimizations (about 50% of possible latency improvement of option 3)
2 - strong latency optimizations (about 75% of possible latency improvement of option 3)
3 - max latency optimizations
4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).

Defaults to None.
 |
| `--output-format` | `alaw_8000 | mp3_22050_32 | mp3_24000_48 | mp3_44100_128 | mp3_44100_192 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | opus_48000_128 | opus_48000_192 | opus_48000_32 | opus_48000_64 | opus_48000_96 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | pcm_8000 | ulaw_8000 | wav_16000 | wav_22050 | wav_24000 | wav_32000 | wav_44100 | wav_48000 | wav_8000` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM and WAV formats with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs speech-to-speech stream`

Stream audio from one voice to another. Maintain full control over emotion, timing and delivery.

`POST /v1/speech-to-speech/{voice_id}/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers. |
| `--optimize-streaming-latency` | `string` | No | You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values:
0 - default mode (no latency optimizations)
1 - normal latency optimizations (about 50% of possible latency improvement of option 3)
2 - strong latency optimizations (about 75% of possible latency improvement of option 3)
3 - max latency optimizations
4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).

Defaults to None.
 |
| `--output-format` | `mp3_22050_32 | mp3_24000_48 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96 | opus_48000_128 | opus_48000_192` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs speech-to-text`

#### `elevenlabs speech-to-text convert`

Transcribe an audio or video file. If webhook is set to true, the request will be processed asynchronously and results sent to configured webhooks. When use_multi_channel is true and the provided audio has multiple channels, a 'transcripts' object with separate transcripts for each channel is returned; set multichannel_output_style='combined' to instead receive a single transcript with all channels merged and sorted by time. Otherwise, returns a single transcript. The optional webhook_metadata parameter allows you to attach custom data that will be included in webhook responses for request correlation and tracking.

`POST /v1/speech-to-text`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--token` | `string` | No | A single-use authentication token created via POST /v1/single-use-token/batch_scribe. This token can only be used once and expires after 15 minutes. Alternative to API key or bearer token authentication for frontend clients. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean log and transcript storage features are unavailable for this request. Zero retention mode may only be used by enterprise customers. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs speech-to-text transcripts`

#### `elevenlabs speech-to-text transcripts delete`

Delete a previously generated transcript by its ID.

`DELETE /v1/speech-to-text/transcripts/{transcription_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--transcription-id` | `string` | Yes | The unique ID of the transcript to delete |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs speech-to-text transcripts get`

Retrieve a previously generated transcript by its ID.

`GET /v1/speech-to-text/transcripts/{transcription_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--transcription-id` | `string` | Yes | The unique ID of the transcript to retrieve |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs studio`

#### `elevenlabs studio create-podcast`

Create and auto-convert a podcast project. Currently, the LLM cost is covered by us but you will still be charged for the audio generation. In the future, you will be charged for both the LLM and audio generation costs.

`POST /v1/studio/podcasts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--safety-identifier` | `string` | No | Used for moderation. Your workspace must be allowlisted to use this feature. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs studio projects`

#### `elevenlabs studio projects convert`

Starts conversion of a Studio project and all of its chapters.

`POST /v1/studio/projects/{project_id}/convert`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects create`

Creates a new Studio project, it can be either initialized as blank, from a document or from a URL.

`POST /v1/studio/projects`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs studio projects delete`

Deletes a Studio project.

`DELETE /v1/studio/projects/{project_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects get`

Returns information about a specific Studio project. This endpoint returns more detailed information about a project than `GET /v1/studio`.

`GET /v1/studio/projects/{project_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--share-id` | `string` | No | The share ID of the project |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects get-muted-tracks`

Returns a list of chapter IDs that have muted tracks in a project.

`GET /v1/studio/projects/{project_id}/muted-tracks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the Studio project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects list`

Returns a list of your Studio projects with metadata.

`GET /v1/studio/projects`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects update`

Updates the specified Studio project by setting the values of the parameters passed.

`POST /v1/studio/projects/{project_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs studio projects chapters`

#### `elevenlabs studio projects chapters convert`

Starts conversion of a specific chapter.

`POST /v1/studio/projects/{project_id}/chapters/{chapter_id}/convert`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--chapter-id` | `string` | Yes | The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects chapters create`

Creates a new chapter either as blank or from a URL.

`POST /v1/studio/projects/{project_id}/chapters`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the Studio project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs studio projects chapters delete`

Deletes a chapter.

`DELETE /v1/studio/projects/{project_id}/chapters/{chapter_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--chapter-id` | `string` | Yes | The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects chapters get`

Returns information about a specific chapter.

`GET /v1/studio/projects/{project_id}/chapters/{chapter_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--chapter-id` | `string` | Yes | The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects chapters list`

Returns a list of a Studio project's chapters.

`GET /v1/studio/projects/{project_id}/chapters`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the Studio project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects chapters update`

Updates a chapter.

`POST /v1/studio/projects/{project_id}/chapters/{chapter_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--chapter-id` | `string` | Yes | The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs studio projects chapters snapshots`

#### `elevenlabs studio projects chapters snapshots get`

Returns the chapter snapshot.

`GET /v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots/{chapter_snapshot_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the Studio project. |
| `--chapter-id` | `string` | Yes | The ID of the chapter. |
| `--chapter-snapshot-id` | `string` | Yes | The ID of the chapter snapshot. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects chapters snapshots list`

Gets information about all the snapshots of a chapter. Each snapshot can be downloaded as audio. Whenever a chapter is converted a snapshot will automatically be created.

`GET /v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--chapter-id` | `string` | Yes | The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects chapters snapshots stream`

Stream the audio from a chapter snapshot. Use `GET /v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots` to return the snapshots of a chapter.

`POST /v1/studio/projects/{project_id}/chapters/{chapter_id}/snapshots/{chapter_snapshot_id}/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--chapter-id` | `string` | Yes | The ID of the chapter to be used. You can use the [List project chapters](/docs/api-reference/studio/get-chapters) endpoint to list all the available chapters. |
| `--chapter-snapshot-id` | `string` | Yes | The ID of the chapter snapshot to be used. You can use the [List project chapter snapshots](/docs/api-reference/studio/get-snapshots) endpoint to list all the available snapshots. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs studio projects content`

#### `elevenlabs studio projects content update`

Updates Studio project content.

`POST /v1/studio/projects/{project_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs studio projects pronunciation-dictionaries`

#### `elevenlabs studio projects pronunciation-dictionaries create`

Create a set of pronunciation dictionaries acting on a project. This will automatically mark text within this project as requiring reconverting where the new dictionary would apply or the old one no longer does.

`POST /v1/studio/projects/{project_id}/pronunciation-dictionaries`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs studio projects snapshots`

#### `elevenlabs studio projects snapshots get`

Returns the project snapshot.

`GET /v1/studio/projects/{project_id}/snapshots/{project_snapshot_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the Studio project. |
| `--project-snapshot-id` | `string` | Yes | The ID of the Studio project snapshot. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects snapshots list`

Retrieves a list of snapshots for a Studio project.

`GET /v1/studio/projects/{project_id}/snapshots`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the Studio project. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs studio projects snapshots stream`

Stream the audio from a Studio project snapshot.

`POST /v1/studio/projects/{project_id}/snapshots/{project_snapshot_id}/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--project-snapshot-id` | `string` | Yes | The ID of the Studio project snapshot. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs studio projects snapshots stream-archive`

Returns a compressed archive of the Studio project's audio.

`POST /v1/studio/projects/{project_id}/snapshots/{project_snapshot_id}/archive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes | The ID of the project to be used. You can use the [List projects](/docs/api-reference/studio/get-projects) endpoint to list all the available projects. |
| `--project-snapshot-id` | `string` | Yes | The ID of the Studio project snapshot. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs text-to-dialogue`

#### `elevenlabs text-to-dialogue convert`

Converts a list of text and voice ID pairs into speech (dialogue) and returns audio.

`POST /v1/text-to-dialogue`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `alaw_8000 | mp3_22050_32 | mp3_24000_48 | mp3_44100_128 | mp3_44100_192 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | opus_48000_128 | opus_48000_192 | opus_48000_32 | opus_48000_64 | opus_48000_96 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | pcm_8000 | ulaw_8000 | wav_16000 | wav_22050 | wav_24000 | wav_32000 | wav_44100 | wav_48000 | wav_8000` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM and WAV formats with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs text-to-dialogue convert-with-timestamps`

Generate dialogue from text with precise character-level timing information for audio-text synchronization.

`POST /v1/text-to-dialogue/with-timestamps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `alaw_8000 | mp3_22050_32 | mp3_24000_48 | mp3_44100_128 | mp3_44100_192 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | opus_48000_128 | opus_48000_192 | opus_48000_32 | opus_48000_64 | opus_48000_96 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | pcm_8000 | ulaw_8000 | wav_16000 | wav_22050 | wav_24000 | wav_32000 | wav_44100 | wav_48000 | wav_8000` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM and WAV formats with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs text-to-dialogue stream`

Converts a list of text and voice ID pairs into speech (dialogue) and returns an audio stream.

`POST /v1/text-to-dialogue/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `AllowedOutputFormats` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs text-to-dialogue stream-with-timestamps`

Converts a list of text and voice ID pairs into speech (dialogue) and returns a stream of JSON blobs containing audio as a base64 encoded string and timestamps

`POST /v1/text-to-dialogue/stream/with-timestamps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `AllowedOutputFormats` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs text-to-sound-effects`

#### `elevenlabs text-to-sound-effects convert`

Turn text into sound effects for your videos, voice-overs or video games using the most advanced sound effects models in the world.

`POST /v1/sound-generation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `AllowedOutputFormats` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs text-to-speech`

#### `elevenlabs text-to-speech convert`

Converts text into speech using a voice of your choice and returns audio.

`POST /v1/text-to-speech/{voice_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers. |
| `--optimize-streaming-latency` | `string` | No | You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values:
0 - default mode (no latency optimizations)
1 - normal latency optimizations (about 50% of possible latency improvement of option 3)
2 - strong latency optimizations (about 75% of possible latency improvement of option 3)
3 - max latency optimizations
4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).

Defaults to None.
 |
| `--output-format` | `alaw_8000 | mp3_22050_32 | mp3_24000_48 | mp3_44100_128 | mp3_44100_192 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | opus_48000_128 | opus_48000_192 | opus_48000_32 | opus_48000_64 | opus_48000_96 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | pcm_8000 | ulaw_8000 | wav_16000 | wav_22050 | wav_24000 | wav_32000 | wav_44100 | wav_48000 | wav_8000` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM and WAV formats with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs text-to-speech convert-with-timestamps`

Generate speech from text with precise character-level timing information for audio-text synchronization.

`POST /v1/text-to-speech/{voice_id}/with-timestamps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers. |
| `--optimize-streaming-latency` | `string` | No | You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values:
0 - default mode (no latency optimizations)
1 - normal latency optimizations (about 50% of possible latency improvement of option 3)
2 - strong latency optimizations (about 75% of possible latency improvement of option 3)
3 - max latency optimizations
4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).

Defaults to None.
 |
| `--output-format` | `alaw_8000 | mp3_22050_32 | mp3_24000_48 | mp3_44100_128 | mp3_44100_192 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | opus_48000_128 | opus_48000_192 | opus_48000_32 | opus_48000_64 | opus_48000_96 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | pcm_8000 | ulaw_8000 | wav_16000 | wav_22050 | wav_24000 | wav_32000 | wav_44100 | wav_48000 | wav_8000` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM and WAV formats with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs text-to-speech stream`

Converts text into speech using a voice of your choice and returns audio as an audio stream.

`POST /v1/text-to-speech/{voice_id}/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers. |
| `--optimize-streaming-latency` | `string` | No | You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values:
0 - default mode (no latency optimizations)
1 - normal latency optimizations (about 50% of possible latency improvement of option 3)
2 - strong latency optimizations (about 75% of possible latency improvement of option 3)
3 - max latency optimizations
4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).

Defaults to None.
 |
| `--output-format` | `mp3_22050_32 | mp3_24000_48 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96 | opus_48000_128 | opus_48000_192` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs text-to-speech stream-with-timestamps`

Converts text into speech using a voice of your choice and returns a stream of JSONs containing audio as a base64 encoded string together with information on when which character was spoken.

`POST /v1/text-to-speech/{voice_id}/stream/with-timestamps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. Use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--enable-logging` | `boolean` | No | When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers. |
| `--optimize-streaming-latency` | `string` | No | You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values:
0 - default mode (no latency optimizations)
1 - normal latency optimizations (about 50% of possible latency improvement of option 3)
2 - strong latency optimizations (about 75% of possible latency improvement of option 3)
3 - max latency optimizations
4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).

Defaults to None.
 |
| `--output-format` | `mp3_22050_32 | mp3_24000_48 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_32000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96 | opus_48000_128 | opus_48000_192` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs text-to-voice`

#### `elevenlabs text-to-voice create`

Create a voice from previously generated voice preview. This endpoint should be called after you fetched a generated_voice_id using POST /v1/text-to-voice/design or POST /v1/text-to-voice/:voice_id/remix.

`POST /v1/text-to-voice`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs text-to-voice create-previews` `[DEPRECATED]`

Create a voice from a text prompt.

`POST /v1/text-to-voice/create-previews`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `AllowedOutputFormats` | No | The output format of the generated audio. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs text-to-voice design`

Design a voice via a prompt. This method returns a list of voice previews. Each preview has a generated_voice_id and a sample of the voice as base64 encoded mp3 audio. To create a voice use the generated_voice_id of the preferred preview with the /v1/text-to-voice endpoint.

`POST /v1/text-to-voice/design`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--output-format` | `AllowedOutputFormats` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs text-to-voice remix`

Remix an existing voice via a prompt. This method returns a list of voice previews. Each preview has a generated_voice_id and a sample of the voice as base64 encoded mp3 audio. To create a voice use the generated_voice_id of the preferred preview with the /v1/text-to-voice endpoint.

`POST /v1/text-to-voice/{voice_id}/remix`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--output-format` | `AllowedOutputFormats` | No | Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs text-to-voice preview`

#### `elevenlabs text-to-voice preview stream`

Stream a voice preview that was created via the /v1/text-to-voice/design endpoint.

`GET /v1/text-to-voice/{generated_voice_id}/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--generated-voice-id` | `string` | Yes | The generated_voice_id to stream. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs tokens single-use`

#### `elevenlabs tokens single-use create`

Generate a time limited single-use token with embedded authentication for frontend clients.

`POST /v1/single-use-token/{token_type}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--token-type` | `SingleUseTokenType` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs usage`

#### `elevenlabs usage get` `[DEPRECATED]`

(Deprecated) This endpoint is deprecated. Use /v1/workspace/analytics/query/usage-by-product-over-time instead, which exposes the bucket size as `interval_seconds` (an integer in seconds) rather than `aggregation_interval`. Returns the usage metrics for the current user or the entire workspace they are part of. The response provides a time axis based on the specified aggregation interval (default: day), with usage values for each interval along that axis. Usage is broken down by the selected breakdown type. For example, breakdown type "voice" will return the usage of each voice for each interval along the time axis.

`GET /v1/usage/character-stats`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--start-unix` | `integer` | Yes | UTC Unix timestamp for the start of the usage window, in milliseconds. To include the first day of the window, the timestamp should be at 00:00:00 of that day. |
| `--end-unix` | `integer` | Yes | UTC Unix timestamp for the end of the usage window, in milliseconds. To include the last day of the window, the timestamp should be at 23:59:59 of that day. |
| `--include-workspace-metrics` | `boolean` | No | Whether or not to include the statistics of the entire workspace. |
| `--breakdown-type` | `BreakdownTypes` | No | How to break down the information. Cannot be "user" if include_workspace_metrics is False. |
| `--aggregation-interval` | `UsageAggregationInterval` | No | How to aggregate usage data over time. Can be "hour", "day", "week", "month", or "cumulative". |
| `--aggregation-bucket-size` | `string` | No | Aggregation bucket size in seconds. Overrides the aggregation interval. |
| `--metric` | `MetricType` | No | Which metric to aggregate. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs user`

#### `elevenlabs user get`

Gets information about the user

`GET /v1/user`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs user subscription`

#### `elevenlabs user subscription get`

Gets extended information about the users subscription

`GET /v1/user/subscription`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs voices`

#### `elevenlabs voices delete`

Deletes a voice by its ID.

`DELETE /v1/voices/{voice_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs voices find-similar-voices`

Returns a list of shared voices similar to the provided audio sample. If neither similarity_threshold nor top_k is provided, we will apply default values.

`POST /v1/similar-voices`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs voices get`

Returns metadata about a specific voice.

`GET /v1/voices/{voice_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--with-settings` | `boolean` | No | This parameter is now deprecated. It is ignored and will be removed in a future version. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs voices get-all` `[DEPRECATED]`

Returns a list of all available voices for a user. Stops working once the user's workspace exceeds 500 voices.

`GET /v1/voices`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--show-legacy` | `string` | No | If set to true, legacy premade voices will be included in responses from /v1/voices |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs voices get-shared`

Retrieves a list of shared voices.

`GET /v1/shared-voices`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--page-size` | `integer` | No | How many shared voices to return at maximum. Can not exceed 100, defaults to 30. |
| `--category` | `professional | famous | high_quality` | No | Voice category used for filtering |
| `--gender` | `string` | No | Gender used for filtering |
| `--age` | `string` | No | Age used for filtering |
| `--accent` | `string` | No | Accent used for filtering |
| `--language` | `string` | No | Language used for filtering |
| `--locale` | `string` | No | Locale used for filtering |
| `--search` | `string` | No | Search term used for filtering |
| `--use-cases` | `string` | No | Use-case used for filtering |
| `--descriptives` | `string` | No | Search term used for filtering |
| `--featured` | `boolean` | No | Filter featured voices |
| `--min-notice-period-days` | `string` | No | Filter voices with a minimum notice period of the given number of days. |
| `--include-custom-rates` | `string` | No | Include/exclude voices with custom rates |
| `--include-live-moderated` | `string` | No | Include/exclude voices that are live moderated |
| `--reader-app-enabled` | `boolean` | No | Filter voices that are enabled for the reader app |
| `--owner-id` | `string` | No | Filter voices by public owner ID |
| `--sort` | `created_date | usage_character_count_1y | trending | cloned_by_count` | No | Sort criteria. Must be one of: created_date, usage_character_count_1y, trending, cloned_by_count. |
| `--page` | `integer` | No |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs voices replicate-to-isolated-environment`

Replicates an Instant Voice Clone or Voice Design voice to a workspace in a different data residency. The target workspace must belong to the same consolidated billing group. The user must have VOICES_WRITE in the source workspace, and be an admin on the source voice. Human users (i.e. not service accounts) must also have VOICES_WRITE in the target workspace. This endpoint is available on the central environment only.

`POST /v1/voices/{voice_id}/replicate-to-isolated-environment`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs voices search`

Gets a list of all available voices for a user with search, filtering and pagination.

`GET /v2/voices`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--next-page-token` | `string` | No | The next page token to use for pagination. Returned from the previous request. Use this in combination with the has_more flag for reliable pagination. |
| `--page-size` | `integer` | No | How many voices to return at maximum. Can not exceed 100, defaults to 10. Page 0 may include more voices due to default voices being included. |
| `--search` | `string` | No | Search term to filter voices by. Searches in name, description, labels, category. |
| `--sort` | `string` | No | Which field to sort by, one of 'created_at_unix' or 'name'. 'created_at_unix' may not be available for older voices. |
| `--sort-direction` | `string` | No | Which direction to sort the voices in. 'asc' or 'desc'. |
| `--voice-type` | `string` | No | Type of the voice to filter by. One of 'personal', 'community', 'default', 'workspace', 'non-default', 'non-community', 'saved'. 'non-default' is equal to all but 'default'. 'non-community' is equal to 'personal' and 'workspace' combined (excludes library copies). 'saved' is equal to non-default, but includes default voices if they have been added to a collection. |
| `--category` | `string` | No | Category of the voice to filter by. One of 'premade', 'cloned', 'generated', 'professional' |
| `--fine-tuning-state` | `string` | No | State of the voice's fine tuning to filter by. Applicable only to professional voices clones. One of 'draft', 'not_verified', 'not_started', 'queued', 'fine_tuning', 'fine_tuned', 'failed', 'delayed' |
| `--collection-id` | `string` | No | Collection ID to filter voices by. |
| `--gender` | `string` | No | Gender used for filtering, based on the voice's 'gender' label. |
| `--age` | `string` | No | Age used for filtering, based on the voice's 'age' label. |
| `--language` | `string` | No | Languages used for filtering, based on the voice's 'language' label. Voices matching any of the given languages are returned. |
| `--accent` | `string` | No | Accent used for filtering, based on the voice's 'accent' label. |
| `--use-cases` | `string` | No | Use cases used for filtering, based on the voice's 'use_case' label. Voices matching any of the given use cases are returned. |
| `--min-notice-period-days` | `string` | No | Filter to voices whose sharing notice period is at least the given number of days. |
| `--include-custom-rates` | `string` | No | Whether to include voices that have a custom sharing rate. Defaults to including them. |
| `--include-live-moderated` | `string` | No | Whether to include voices that have live moderation enabled. Defaults to including them. |
| `--high-quality` | `string` | No | When true, only return studio-quality voices (those whose category is 'high_quality'). |
| `--include-total-count` | `boolean` | No | Whether to include the total count of voices found in the response. NOTE: The total_count value is a live snapshot and may change between requests as users create, modify, or delete voices. For pagination, rely on the has_more flag instead. Only enable this when you actually need the total count (e.g., for display purposes), as it incurs a performance cost. |
| `--voice-ids` | `string` | No | Voice IDs to lookup by. Maximum 100 voice IDs. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs voices share`

Add a shared voice to your collection of Voices

`POST /v1/voices/add/{public_user_id}/{voice_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--public-user-id` | `string` | Yes | Public user ID used to publicly identify ElevenLabs users. |
| `--voice-id` | `string` | Yes | ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs voices update`

Edit a voice created by you.

`POST /v1/voices/{voice_id}/edit`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs voices accents`

#### `elevenlabs voices accents get`

Gets the list of available accents in the shared voice library.

`GET /v1/voices/accents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--language` | `string` | No | If provided, only accents for this language code are returned. |
| `--model-id` | `string` | No | If provided, returns the accents available for this model. Defaults to the most complete accent list when omitted. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs voices ivc`

#### `elevenlabs voices ivc create`

Create a voice clone and add it to your Voices

`POST /v1/voices/add`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs voices pvc`

#### `elevenlabs voices pvc create`

Creates a new PVC voice with metadata but no samples

`POST /v1/voices/pvc`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs voices pvc train`

Start PVC training process for a voice.

`POST /v1/voices/pvc/{voice_id}/train`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs voices pvc update`

Edit PVC voice metadata

`POST /v1/voices/pvc/{voice_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs voices pvc samples`

#### `elevenlabs voices pvc samples create`

Add audio samples to a PVC voice

`POST /v1/voices/pvc/{voice_id}/samples`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs voices pvc samples delete`

Delete a sample from a PVC voice.

`DELETE /v1/voices/pvc/{voice_id}/samples/{sample_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--sample-id` | `string` | Yes | Sample ID to be used |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs voices pvc samples update`

Update a PVC voice sample - apply noise removal, select speaker, change trim times or file name.

`POST /v1/voices/pvc/{voice_id}/samples/{sample_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--sample-id` | `string` | Yes | Sample ID to be used |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs voices pvc samples audio`

#### `elevenlabs voices pvc samples audio get`

Retrieve the first 30 seconds of voice sample audio with or without noise removal.

`GET /v1/voices/pvc/{voice_id}/samples/{sample_id}/audio`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--sample-id` | `string` | Yes | Sample ID to be used |
| `--remove-background-noise` | `boolean` | No | If set will remove background noise for voice samples using our audio isolation model. If the samples do not include background noise, it can make the quality worse. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs voices pvc samples speakers`

#### `elevenlabs voices pvc samples speakers get`

Retrieve the status of the speaker separation process and the list of detected speakers if complete.

`GET /v1/voices/pvc/{voice_id}/samples/{sample_id}/speakers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--sample-id` | `string` | Yes | Sample ID to be used |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs voices pvc samples speakers separate`

Start speaker separation process for a sample

`POST /v1/voices/pvc/{voice_id}/samples/{sample_id}/separate-speakers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--sample-id` | `string` | Yes | Sample ID to be used |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs voices pvc samples speakers audio`

#### `elevenlabs voices pvc samples speakers audio get`

Retrieve the separated audio for a specific speaker.

`GET /v1/voices/pvc/{voice_id}/samples/{sample_id}/speakers/{speaker_id}/audio`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--sample-id` | `string` | Yes | Sample ID to be used |
| `--speaker-id` | `string` | Yes | Speaker ID to be used, you can use GET https://api.elevenlabs.io/v1/voices/{voice_id}/samples/{sample_id}/speakers to list all the available speakers for a sample. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs voices pvc samples waveform`

#### `elevenlabs voices pvc samples waveform get`

Retrieve the visual waveform of a voice sample.

`GET /v1/voices/pvc/{voice_id}/samples/{sample_id}/waveform`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--sample-id` | `string` | Yes | Sample ID to be used |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs voices pvc verification`

#### `elevenlabs voices pvc verification request`

Request manual verification for a PVC voice.

`POST /v1/voices/pvc/{voice_id}/verification`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs voices pvc verification captcha`

#### `elevenlabs voices pvc verification captcha get`

Get captcha for PVC voice verification.

`GET /v1/voices/pvc/{voice_id}/captcha`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs voices pvc verification captcha verify`

Submit captcha verification for PVC voice.

`POST /v1/voices/pvc/{voice_id}/captcha`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs voices samples audio`

#### `elevenlabs voices samples audio get`

Returns the audio corresponding to a sample attached to a voice.

`GET /v1/voices/{voice_id}/samples/{sample_id}/audio`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--sample-id` | `string` | Yes | ID of the sample to be used. You can use the [Get voices](/docs/api-reference/voices/get) endpoint list all the available samples for a voice. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs voices settings`

#### `elevenlabs voices settings get`

Returns the settings for a specific voice. "similarity_boost" corresponds to"Clarity + Similarity Enhancement" in the web app and "stability" corresponds to "Stability" slider in the web app.

`GET /v1/voices/{voice_id}/settings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | Voice ID to be used, you can use https://api.elevenlabs.io/v1/voices to list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs voices settings get-default`

Gets the default settings for voices. "similarity_boost" corresponds to"Clarity + Similarity Enhancement" in the web app and "stability" corresponds to "Stability" slider in the web app.

`GET /v1/voices/settings/default`

#### `elevenlabs voices settings update`

Edit your settings for a specific voice. "similarity_boost" corresponds to "Clarity + Similarity Enhancement" in the web app and "stability" corresponds to "Stability" slider in the web app.

`POST /v1/voices/{voice_id}/settings/edit`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--voice-id` | `string` | Yes | ID of the voice to be used. You can use the [Get voices](/docs/api-reference/voices/search) endpoint list all the available voices. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs webhooks`

#### `elevenlabs webhooks create`

Create a new webhook for the workspace with the specified authentication type.

`POST /v1/workspace/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs webhooks delete`

Delete the specified workspace webhook

`DELETE /v1/workspace/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--webhook-id` | `string` | Yes | The unique ID for the webhook |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs webhooks list`

List all webhooks for a workspace

`GET /v1/workspace/webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--include-usages` | `boolean` | No | Whether to include active usages of the webhook, only usable by admins |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs webhooks update`

Update the specified workspace webhook

`PATCH /v1/workspace/webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--webhook-id` | `string` | Yes | The unique ID for the webhook |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs workspace`

#### `elevenlabs workspace set-third-party-disabling-policy`

Set the workspace-wide Third-Party Disabling policy. When set, it forces, for every API key in the workspace, whether the holder of a key (potentially a third party who found it) may disable it via the self-disable endpoint or when it leaks publicly — overriding each key's own setting. Pass `true` to allow it for all keys, `false` to forbid it for all keys, or `null` to clear the override so per-key values and the plan default apply again. Workspace admins only.

`POST /v1/workspaces/api-keys/third-party-disabling`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs workspace analytics requests`

#### `elevenlabs workspace analytics requests get`

Returns a list of API requests. Supports filtering by time range, column filters, and search terms. At least one of start_time or end_time must be provided. An optional sort parameter controls timestamp ordering. Results are ordered by timestamp. Descending if end_time is used, ascending if start_time is used. The response is a tabular structure with columns, column_types, column_units, and rows.

`POST /v1/workspace/analytics/requests`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs workspace audit-logs`

#### `elevenlabs workspace audit-logs list`

Returns the audit log for the workspace. Requires enterprise tier and the audit_log_read permission.

`GET /v1/workspace/audit-logs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer` | No | Maximum number of entries per page |
| `--cursor` | `string` | No | Cursor for the next page (from previous response) |
| `--time-from-unix-ms` | `string` | No | Only include entries at or after this time (ms since epoch) |
| `--time-to-unix-ms` | `string` | No | Only include entries at or before this time (ms since epoch) |
| `--actor-uid` | `string` | No | Filter by actor user ID |
| `--class-name` | `string` | No | Filter by OCSF event class name (e.g. Account Change) |
| `--activity-name` | `string` | No | Filter by audit activity name (e.g. Subscription Creation) |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs workspace auth-connections`

#### `elevenlabs workspace auth-connections create`

Create a new OAuth2 auth connection for the workspace

`POST /v1/workspace/auth-connections`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs workspace auth-connections delete`

Delete an auth connection

`DELETE /v1/workspace/auth-connections/{auth_connection_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--auth-connection-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs workspace auth-connections list`

Get all auth connections for the workspace

`GET /v1/workspace/auth-connections`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs workspace auth-connections update`

Update an auth connection

`PATCH /v1/workspace/auth-connections/{auth_connection_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--auth-connection-id` | `string` | Yes |  |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs workspace groups`

#### `elevenlabs workspace groups list`

Get all groups in the workspace

`GET /v1/workspace/groups`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs workspace groups search`

Searches for user groups in the workspace. Multiple or no groups may be returned.

`GET /v1/workspace/groups/search`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name` | `string` | Yes | Name of the target group. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

### `elevenlabs workspace groups members`

#### `elevenlabs workspace groups members add`

Adds a member of your workspace to the specified group. Requires `group_members_manage` permission.

`POST /v1/workspace/groups/{group_id}/members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--group-id` | `string` | Yes | The ID of the target group. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs workspace groups members remove`

Removes a member from the specified group. Requires `group_members_manage` permission.

`POST /v1/workspace/groups/{group_id}/members/remove`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--group-id` | `string` | Yes | The ID of the target group. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs workspace invites`

#### `elevenlabs workspace invites create`

Sends an email invitation to join your workspace to the provided email. If the user doesn't have an account they will be prompted to create one. If the user accepts this invite they will be added as a user to your workspace and your subscription using one of your seats. This endpoint may only be called by workspace members with the WORKSPACE_MEMBERS_INVITE permission. If the user is already in the workspace a 400 error will be returned.

`POST /v1/workspace/invites/add`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs workspace invites create-batch`

Sends email invitations to join your workspace to the provided emails. Requires all email addresses to be part of a verified domain. If the users don't have an account they will be prompted to create one. If the users accept these invites they will be added as users to your workspace and your subscription using one of your seats. This endpoint may only be called by workspace members with the WORKSPACE_MEMBERS_INVITE permission.

`POST /v1/workspace/invites/add-bulk`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs workspace invites delete`

Invalidates an existing email invitation. The invitation will still show up in the inbox it has been delivered to, but activating it to join the workspace won't work. This endpoint may only be called by workspace members with the WORKSPACE_MEMBERS_INVITE permission.

`DELETE /v1/workspace/invites`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs workspace members`

#### `elevenlabs workspace members list`

Gets a list of all members of the workspace, including locked members. Service accounts are excluded. Requires the workspace_members_read permission.

`GET /v1/workspace/members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs workspace members update`

Updates attributes of a workspace member. Apart from the email identifier, all parameters will remain unchanged unless specified. This endpoint may only be called by workspace administrators.

`POST /v1/workspace/members`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs workspace resources`

#### `elevenlabs workspace resources get`

Gets the metadata of a resource by ID.

`GET /v1/workspace/resources/{resource_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--resource-id` | `string` | Yes | The ID of the target resource. |
| `--resource-type` | `WorkspaceResourceType` | Yes | Resource type of the target resource. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

#### `elevenlabs workspace resources share`

Grants a role (one of 'admin', 'editor', 'commenter', or 'viewer') on a workspace resource to a user, group, or workspace (service account) API key. This overrides any existing role the target has on the resource. To target a user or service account, pass only the user email; the user must be in your workspace. To target a group, pass only the group id. To target a workspace (service account) API key, pass the api key id; the resource will be shared with the service account associated with that key. You must have admin access to the resource to share it.

`POST /v1/workspace/resources/{resource_id}/share`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--resource-id` | `string` | Yes | The ID of the target resource. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `elevenlabs workspace resources unshare`

Removes any existing role on a workspace resource from a user, group, or workspace (service account) API key. To target a user or service account, pass only the user email; the user must be in your workspace. To target a group, pass only the group id. To target a workspace (service account) API key, pass the api key id; the resource will be unshared from the service account associated with that key. You must have admin access to the resource to unshare it. You cannot remove permissions from the user who created the resource.

`POST /v1/workspace/resources/{resource_id}/unshare`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--resource-id` | `string` | Yes | The ID of the target resource. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs workspace usage`

#### `elevenlabs workspace usage get-usage-by-product-over-time`

Returns credit usage broken down by product type over time. The response is a tabular structure with columns, column_types, column_units, and rows.

`POST /v1/workspace/analytics/query/usage-by-product-over-time`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `elevenlabs workspaces api-keys`

#### `elevenlabs workspaces api-keys disable`

Disable the API key used to authenticate this request. Requires the query parameter `api_key_name=self` as an explicit confirmation.

`POST /v1/workspaces/api-keys/disable`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--api-key-name` | `string` | Yes | Must be set to `self` to disable the API key used to authenticate this request. Required as an explicit confirmation to avoid accidentally disabling the wrong key. |
| `--xi-api-key` | `string` | No | Your API key. This is required by most endpoints to access our API programmatically. You can view your xi-api-key using the 'Profile' tab on the website. |

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

