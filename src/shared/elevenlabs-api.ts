import { ElevenLabsClient, ElevenLabsError } from '@elevenlabs/elevenlabs-js';
import { ElevenLabs } from '@elevenlabs/elevenlabs-js';
import * as core from '@elevenlabs/elevenlabs-js/core';
import {
  ConversationalConfig,
  AgentPlatformSettingsRequestModel,
  AgentWorkflowRequestModel
} from '@elevenlabs/elevenlabs-js/api';
import { getApiKey, loadConfig, Location } from './config.js';
import { toCamelCaseKeys, toSnakeCaseKeys } from './utils.js';

type Supplier<T> = T | Promise<T> | (() => T | Promise<T>);
type HeaderValue = string | Supplier<string | null | undefined> | null | undefined;

interface ClientRuntimeOptions {
  apiKey?: Supplier<string | undefined>;
  baseUrl?: Supplier<string>;
  headers?: Record<string, HeaderValue>;
  timeoutInSeconds?: number;
  maxRetries?: number;
  fetch?: typeof globalThis.fetch;
  fetcher?: core.FetchFunction;
  logging?: core.Fetcher.Args['logging'];
}

interface ClientWithRuntimeOptions {
  _options?: ClientRuntimeOptions;
}

interface RawAgentApiResponse {
  agentId?: string;
  versionId?: string;
  branchId?: string;
}

const SDK_MISSING_WORKFLOW_NODE_TYPES = new Set(['say']);

// Type guard for conversational config
function isConversationalConfig(config: unknown): config is ConversationalConfig {
  return typeof config === 'object' && config !== null;
}

// Type guard for platform settings
function isPlatformSettings(settings: unknown): settings is AgentPlatformSettingsRequestModel {
  return typeof settings === 'object' && settings !== null;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function workflowHasSdkMissingNode(workflow: unknown): boolean {
  if (!isRecord(workflow) || !isRecord(workflow.nodes)) {
    return false;
  }

  return Object.values(workflow.nodes).some((node) => {
    if (!isRecord(node) || typeof node.type !== 'string') {
      return false;
    }

    return SDK_MISSING_WORKFLOW_NODE_TYPES.has(node.type);
  });
}

async function resolveSupplier<T>(supplier: Supplier<T> | undefined): Promise<T | undefined> {
  if (typeof supplier === 'function') {
    return await (supplier as () => T | Promise<T>)();
  }

  return await supplier;
}

async function resolveHeaders(headers: Record<string, HeaderValue> | undefined): Promise<Record<string, string>> {
  const resolvedHeaders: Record<string, string> = {};

  if (!headers) {
    return resolvedHeaders;
  }

  for (const [key, value] of Object.entries(headers)) {
    const resolvedValue = await resolveSupplier(value);
    if (resolvedValue != null) {
      resolvedHeaders[key] = resolvedValue;
    }
  }

  return resolvedHeaders;
}

async function getClientRuntimeOptions(client: ElevenLabsClient): Promise<{
  baseUrl: string;
  headers: Record<string, string>;
  timeoutInSeconds?: number;
  maxRetries?: number;
  fetch?: typeof globalThis.fetch;
  fetcher: core.FetchFunction;
  logging?: core.Fetcher.Args['logging'];
}> {
  const options = (client as unknown as ClientWithRuntimeOptions)._options;
  const config = await loadConfig();
  const baseUrl = await resolveSupplier(options?.baseUrl) ?? getApiBaseUrl(config.residency);
  const headers = await resolveHeaders(options?.headers);
  const apiKey = await resolveSupplier(options?.apiKey) ?? await getApiKey();

  if (apiKey) {
    headers['xi-api-key'] = apiKey;
  }

  return {
    baseUrl,
    headers,
    timeoutInSeconds: options?.timeoutInSeconds,
    maxRetries: options?.maxRetries,
    fetch: options?.fetch,
    fetcher: options?.fetcher ?? core.fetcher,
    logging: options?.logging
  };
}

function throwRawAgentApiError(
  response: core.APIResponse<unknown, core.Fetcher.Error>,
  method: string,
  path: string
): never {
  if (response.ok) {
    throw new Error('Cannot throw an API error for a successful response');
  }

  if (response.error.reason === 'status-code') {
    if (response.error.statusCode === 422) {
      throw new ElevenLabs.UnprocessableEntityError(response.error.body, response.rawResponse);
    }

    throw new ElevenLabsError({
      message: `${method} ${path} failed with status ${response.error.statusCode}`,
      statusCode: response.error.statusCode,
      body: response.error.body,
      rawResponse: response.rawResponse
    });
  }

  throw new ElevenLabsError({
    message: `${method} ${path} failed: ${response.error.reason}`,
    rawResponse: response.rawResponse
  });
}

async function rawAgentRequest(
  client: ElevenLabsClient,
  method: 'POST' | 'PATCH',
  path: string,
  body: Record<string, unknown>,
  queryParameters?: Record<string, unknown>
): Promise<RawAgentApiResponse> {
  const options = await getClientRuntimeOptions(client);
  const response = await options.fetcher({
    url: core.url.join(options.baseUrl, path),
    method,
    headers: options.headers,
    contentType: 'application/json',
    queryParameters,
    requestType: 'json',
    responseType: 'json',
    body,
    timeoutMs: (options.timeoutInSeconds ?? 240) * 1000,
    maxRetries: options.maxRetries,
    fetchFn: options.fetch,
    logging: options.logging
  });

  if (!response.ok) {
    throwRawAgentApiError(response, method, path);
  }

  return toCamelCaseKeys(response.body) as RawAgentApiResponse;
}

/**
 * Cleans conversation config before sending to API.
 * Removes the deprecated 'tools' field if 'tool_ids' is present to avoid API conflicts.
 * The API returns both fields, but only accepts one when creating/updating.
 */
function cleanConversationConfigForApi(config: Record<string, unknown>): Record<string, unknown> {
  const cleaned = { ...config };

  // Handle nested agent.prompt structure
  if (cleaned.agent && typeof cleaned.agent === 'object') {
    const agent = { ...(cleaned.agent as Record<string, unknown>) };

    if (agent.prompt && typeof agent.prompt === 'object') {
      const prompt = { ...(agent.prompt as Record<string, unknown>) };

      // If tool_ids exists, remove tools (deprecated field) to avoid API error
      if (prompt.tool_ids !== undefined || prompt.toolIds !== undefined) {
        delete prompt.tools;
      }

      agent.prompt = prompt;
    }

    cleaned.agent = agent;
  }

  return cleaned;
}
/**
 * Gets the API base URL based on residency configuration
 */
export function getApiBaseUrl(residency?: Location): string {
  switch (residency) {
    case 'eu-residency':
      return 'https://api.eu.residency.elevenlabs.io';
    case 'in-residency':
      return 'https://api.in.residency.elevenlabs.io';
    case 'us':
      return 'https://api.us.elevenlabs.io';
    case 'global':
    default:
      return 'https://api.elevenlabs.io';
  }
}

/**
 * Retrieves the ElevenLabs API key from config or environment variables and returns an API client.
 *
 * @throws {Error} If no API key is found
 * @returns An instance of the ElevenLabs client
 */
export async function getElevenLabsClient(): Promise<ElevenLabsClient> {
  const apiKey = await getApiKey();
  if (!apiKey) {
    throw new Error(`No API key found. Use 'elevenlabs auth login' to authenticate or set ELEVENLABS_API_KEY environment variable.`);
  }
  
  const config = await loadConfig();
  const baseURL = getApiBaseUrl(config.residency);
  
  return new ElevenLabsClient({ 
    apiKey,
    baseUrl: baseURL,
    headers: {
      'X-Source': 'agents-cli'
    }
  });
}

/**
 * Creates a new agent using the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param name - The name of the agent
 * @param conversationConfigDict - A dictionary for ConversationalConfig
 * @param platformSettingsDict - An optional dictionary for AgentPlatformSettings
 * @param workflow - An optional workflow configuration
 * @param tags - An optional list of tags
 * @returns Promise that resolves to the agent_id of the newly created agent
 */
export async function createAgentApi(
  client: ElevenLabsClient,
  name: string,
  conversationConfigDict: Record<string, unknown>,
  platformSettingsDict?: Record<string, unknown>,
  workflow?: unknown,
  tags?: string[]
): Promise<string> {
  if (!isConversationalConfig(conversationConfigDict)) {
    throw new Error('Invalid conversation config provided');
  }

  // Clean config to remove deprecated 'tools' if 'tool_ids' exists
  const cleanedConfig = cleanConversationConfigForApi(conversationConfigDict);

  // Normalize to camelCase for API
  const convConfig = toCamelCaseKeys(cleanedConfig) as ConversationalConfig;
  const platformSettings = platformSettingsDict && isPlatformSettings(platformSettingsDict) ? toCamelCaseKeys(platformSettingsDict) as AgentPlatformSettingsRequestModel : undefined;

  // Normalize workflow to camelCase for API (same as conversationConfig and platformSettings)
  const workflowConfig = workflow ? toCamelCaseKeys(workflow) as AgentWorkflowRequestModel : undefined;

  if (workflowHasSdkMissingNode(workflowConfig)) {
    const response = await rawAgentRequest(client, 'POST', 'v1/convai/agents/create', toSnakeCaseKeys({
      name,
      conversationConfig: convConfig,
      platformSettings,
      workflow: workflowConfig,
      tags
    }));

    if (!response.agentId) {
      throw new Error('Create agent response did not include agentId');
    }

    return response.agentId;
  }

  const response = await client.conversationalAi.agents.create({
    name,
    conversationConfig: convConfig,
    platformSettings,
    workflow: workflowConfig,
    tags
  });

  return response.agentId;
}

/**
 * Updates an existing agent using the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param agentId - The ID of the agent to update
 * @param name - Optional new name for the agent
 * @param conversationConfigDict - Optional new dictionary for ConversationalConfig
 * @param platformSettingsDict - Optional new dictionary for AgentPlatformSettings
 * @param workflow - Optional workflow configuration
 * @param tags - Optional new list of tags
 * @returns Promise that resolves to the agent_id of the updated agent
 */
export async function updateAgentApi(
  client: ElevenLabsClient,
  agentId: string,
  name?: string,
  conversationConfigDict?: Record<string, unknown>,
  platformSettingsDict?: Record<string, unknown>,
  workflow?: unknown,
  tags?: string[],
  versionDescription?: string,
  branchId?: string
): Promise<{ agentId: string; versionId?: string; branchId?: string }> {
  // Clean config to remove deprecated 'tools' if 'tool_ids' exists
  const cleanedConfig = conversationConfigDict ? cleanConversationConfigForApi(conversationConfigDict) : undefined;

  const convConfig = cleanedConfig && isConversationalConfig(cleanedConfig) ? toCamelCaseKeys(cleanedConfig) as ConversationalConfig : undefined;
  const platformSettings = platformSettingsDict && isPlatformSettings(platformSettingsDict) ? toCamelCaseKeys(platformSettingsDict) as AgentPlatformSettingsRequestModel : undefined;
  // Normalize workflow to camelCase for API (same as conversationConfig and platformSettings)
  const workflowConfig = workflow ? toCamelCaseKeys(workflow) as AgentWorkflowRequestModel : undefined;

  if (workflowHasSdkMissingNode(workflowConfig)) {
    const response = await rawAgentRequest(
      client,
      'PATCH',
      `v1/convai/agents/${core.url.encodePathParam(agentId)}`,
      toSnakeCaseKeys({
        name,
        conversationConfig: convConfig,
        platformSettings,
        workflow: workflowConfig,
        tags,
        versionDescription
      }),
      branchId ? { branch_id: branchId } : undefined
    );

    return {
      agentId: response.agentId ?? agentId,
      versionId: response.versionId,
      branchId: response.branchId
    };
  }

  const response = await client.conversationalAi.agents.update(agentId, {
    name,
    conversationConfig: convConfig,
    platformSettings,
    workflow: workflowConfig,
    tags,
    versionDescription,
    ...(branchId ? { branchId } : {})
  });

  return {
    agentId: response.agentId,
    versionId: response.versionId,
    branchId: response.branchId
  };
}

/**
 * Lists all agents from the ElevenLabs API.
 * 
 * @param client - An initialized ElevenLabs client
 * @param pageSize - Maximum number of agents to return per page (default: 30, max: 100)
 * @param search - Optional search string to filter agents by name
 * @returns Promise that resolves to a list of agent metadata objects
 */
export async function listAgentsApi(
  client: ElevenLabsClient,
  pageSize: number = 30,
  search?: string
): Promise<unknown[]> {
  const allAgents: unknown[] = [];
  let cursor: string | undefined;
  
  while (true) {
    const requestParams: Record<string, unknown> = {
      pageSize: Math.min(pageSize, 100)
    };
    
    if (cursor) {
      requestParams.cursor = cursor;
    }
    
    if (search) {
      requestParams.search = search;
    }
    
    const response = await client.conversationalAi.agents.list(requestParams);
    
    allAgents.push(...response.agents);
    
    if (!response.hasMore) {
      break;
    }
    
    cursor = response.nextCursor;
  }
  
  return allAgents;
}

/**
 * Gets detailed configuration for a specific agent from the ElevenLabs API.
 * 
 * @param client - An initialized ElevenLabs client
 * @param agentId - The ID of the agent to retrieve
 * @returns Promise that resolves to an object containing the full agent configuration
 */
export async function getAgentApi(client: ElevenLabsClient, agentId: string, branchId?: string): Promise<unknown> {
  const response = branchId
    ? await client.conversationalAi.agents.get(agentId, { branchId })
    : await client.conversationalAi.agents.get(agentId);
  // Normalize response to snake_case for downstream writing
  return toSnakeCaseKeys(response);
}

/**
 * Lists branches for a specific agent from the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param agentId - The ID of the agent
 * @param includeArchived - Whether to include archived branches (default: false)
 * @returns Promise that resolves to a list of branch summary objects
 */
export async function listBranchesApi(
  client: ElevenLabsClient,
  agentId: string,
  includeArchived: boolean = false
): Promise<ElevenLabs.AgentBranchSummary[]> {
  const response = await client.conversationalAi.agents.branches.list(agentId, {
    includeArchived
  });
  return response.results;
}

/**
 * Resolves a branch name or ID to a branch ID.
 * If the input starts with 'agtbrch_', it's treated as an ID directly.
 * Otherwise, it's treated as a branch name and resolved via the branches list.
 *
 * @param client - An initialized ElevenLabs client
 * @param agentId - The ID of the agent
 * @param branchNameOrId - Branch name or ID to resolve
 * @returns Promise that resolves to the branch ID
 */
export async function resolveBranchId(
  client: ElevenLabsClient,
  agentId: string,
  branchNameOrId: string
): Promise<string> {
  // If it looks like a branch ID, return it directly
  if (branchNameOrId.startsWith('agtbrch_')) {
    return branchNameOrId;
  }

  // Otherwise, resolve name to ID (include archived so resolution doesn't silently fail)
  const branches = await listBranchesApi(client, agentId, true);
  const match = branches.find(b => b.name === branchNameOrId);
  if (!match) {
    throw new Error(
      `Branch '${branchNameOrId}' not found for agent '${agentId}'. ` +
      `Use 'elevenlabs agents branches list --agent ${agentId}' to see available branches.`
    );
  }
  return match.id;
}

/**
 * Deletes an agent using the ElevenLabs API.
 * 
 * @param client - An initialized ElevenLabs client
 * @param agentId - The ID of the agent to delete
 * @returns Promise that resolves when the agent is deleted
 */
export async function deleteAgentApi(client: ElevenLabsClient, agentId: string): Promise<void> {
  await client.conversationalAi.agents.delete(agentId);
}

/**
 * Deletes a tool using the ElevenLabs API.
 * 
 * @param client - An initialized ElevenLabs client
 * @param toolId - The ID of the tool to delete
 * @returns Promise that resolves when the tool is deleted
 */
export async function deleteToolApi(client: ElevenLabsClient, toolId: string): Promise<void> {
  await client.conversationalAi.tools.delete(toolId);
}

/**
 * Deletes a test using the ElevenLabs API.
 * 
 * @param client - An initialized ElevenLabs client
 * @param testId - The ID of the test to delete
 * @returns Promise that resolves when the test is deleted
 */
export async function deleteTestApi(client: ElevenLabsClient, testId: string): Promise<void> {
  await client.conversationalAi.tests.delete(testId);
}

/**
 * Creates a new tool using the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param toolConfig - The tool configuration object
 * @returns Promise that resolves to the created tool object
 */
export async function createToolApi(client: ElevenLabsClient, toolConfig: Record<string, unknown>): Promise<ElevenLabs.ToolResponseModel> {
  const normalizedConfig = toCamelCaseKeys(toolConfig);

  return await client.conversationalAi.tools.create({
    toolConfig: normalizedConfig as unknown as ElevenLabs.ToolRequestModelToolConfig
  })
}

/**
 * Updates an existing tool using the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param toolId - The ID of the tool to update
 * @param toolConfig - The updated tool configuration object
 * @returns Promise that resolves to the updated tool object
 */
export async function updateToolApi(client: ElevenLabsClient, toolId: string, toolConfig: Record<string, unknown>): Promise<ElevenLabs.ToolResponseModel> {
  // Normalize to camelCase for API
  const normalizedConfig = toCamelCaseKeys(toolConfig);

  return await client.conversationalAi.tools.update(toolId, {
    toolConfig: normalizedConfig as unknown as ElevenLabs.ToolRequestModelToolConfig
  })
}

/**
 * Gets a specific tool from the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param toolId - The ID of the tool to retrieve
 * @returns Promise that resolves to the tool object
 */
export async function getToolApi(client: ElevenLabsClient, toolId: string): Promise<unknown> {
  const response = await client.conversationalAi.tools.get(toolId);
  // Normalize response to snake_case for downstream writing
  return toSnakeCaseKeys(response);
}

/**
 * Lists all tools from the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @returns Promise that resolves to a list of tool objects
 */
export async function listToolsApi(client: ElevenLabsClient): Promise<unknown[]> {
  const response = await client.conversationalAi.tools.list();
  return response.tools.map(tool => toSnakeCaseKeys(tool));
}

/**
 * Gets agents that depend on a specific tool.
 *
 *
 * @param client - An initialized ElevenLabs client
 * @param toolId - The ID of the tool
 * @returns Promise that resolves to a list of dependent agents
 */
export async function getToolDependentAgentsApi(client: ElevenLabsClient, toolId: string): Promise<unknown[]> {
  const response = await client.conversationalAi.tools.getDependentAgents(toolId);
  return response.agents.map(agent => toSnakeCaseKeys(agent));
}

// Test API functions

/**
 * Creates a new test using the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param testConfig - The test configuration object
 * @returns Promise that resolves to the created test with ID
 */
export async function createTestApi(client: ElevenLabsClient, testConfig: ElevenLabs.conversationalAi.TestsCreateRequestBody): Promise<{ id: string }> {
  const response = await client.conversationalAi.tests.create(testConfig);
  return response as { id: string };
}

/**
 * Gets a specific test from the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param testId - The ID of the test to retrieve
 * @returns Promise that resolves to the test object
 */
export async function getTestApi(client: ElevenLabsClient, testId: string): Promise<unknown> {
  const response = await client.conversationalAi.tests.get(testId);
  return toSnakeCaseKeys(response);
}

/**
 * Lists all tests from the ElevenLabs API, paginating through every page.
 *
 * The underlying SDK endpoint returns at most `pageSize` entries per call (cap 100).
 * This helper loops using the response cursor until `hasMore` is false, so callers
 * always receive the complete set regardless of how many tests exist.
 *
 * @param client - An initialized ElevenLabs client
 * @param pageSize - Page size for each API call (default: 100, SDK max)
 * @returns Promise that resolves to every test in the workspace
 */
export async function listTestsApi(client: ElevenLabsClient, pageSize: number = 100): Promise<unknown[]> {
  const allTests: unknown[] = [];
  let cursor: string | undefined;
  while (true) {
    const request: { pageSize: number; cursor?: string } = { pageSize };
    if (cursor) request.cursor = cursor;
    const response = await client.conversationalAi.tests.list(request) as {
      tests?: unknown[];
      nextCursor?: string;
      hasMore?: boolean;
    };
    if (response.tests?.length) allTests.push(...response.tests);
    if (!response.hasMore) break;
    cursor = response.nextCursor;
    if (!cursor) break;
  }
  return allTests;
}

/**
 * Updates an existing test using the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param testId - The ID of the test to update
 * @param testConfig - The updated test configuration object
 * @returns Promise that resolves to the updated test object
 */
export async function updateTestApi(client: ElevenLabsClient, testId: string, testConfig: ElevenLabs.conversationalAi.TestsUpdateRequestBody): Promise<unknown> {
  const response = await client.conversationalAi.tests.update(testId, testConfig);
  return toSnakeCaseKeys(response);
}

/**
 * Runs tests on an agent using the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param agentId - The ID of the agent to run tests on
 * @param testIds - Array of test IDs to run
 * @param agentConfigOverride - Optional agent configuration override
 * @returns Promise that resolves to the test invocation with ID
 */
export async function runTestsOnAgentApi(
  client: ElevenLabsClient,
  agentId: string,
  testIds: string[],
  agentConfigOverride?: Record<string, unknown>
): Promise<unknown> {
  const tests = testIds.map(testId => ({ testId }));
  const requestBody: ElevenLabs.conversationalAi.RunAgentTestsRequestModel = { tests };

  if (agentConfigOverride) {
    requestBody.agentConfigOverride = agentConfigOverride as unknown as ElevenLabs.AdhocAgentConfigOverrideForTestRequestModel;
  }

  const response = await client.conversationalAi.agents.runTests(agentId, requestBody);
  return toSnakeCaseKeys(response);
}

/**
 * Gets test invocation results from the ElevenLabs API.
 *
 * @param client - An initialized ElevenLabs client
 * @param testInvocationId - The ID of the test invocation
 * @returns Promise that resolves to the test invocation results
 */
export async function getTestInvocationApi(client: ElevenLabsClient, testInvocationId: string): Promise<unknown> {
  const response = await client.conversationalAi.tests.invocations.get(testInvocationId);
  return toSnakeCaseKeys(response);
} 