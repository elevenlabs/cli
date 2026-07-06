import { ElevenLabsClient } from '@elevenlabs/elevenlabs-js';
import { ElevenLabs } from '@elevenlabs/elevenlabs-js';
import { getApiKey, loadConfig, Location } from './config.js';
import { toCamelCaseKeys, toSnakeCaseKeys } from './utils.js';

/**
 * Cleans conversation config before sending to API.
 * Removes the deprecated 'tools' field if 'tool_ids' is present to avoid API conflicts.
 * The API returns both fields, but only accepts one when creating/updating.
 */
export function cleanConversationConfigForApi(config: Record<string, unknown>): Record<string, unknown> {
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
    case 'sg-residency':
      return 'https://api.sg.residency.elevenlabs.io';
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
 * Connection details for raw Convai API requests.
 */
export interface ApiContext {
  apiKey: string;
  baseUrl: string;
}

/**
 * Resolves the API key and base URL for raw Convai API requests.
 *
 * @throws {Error} If no API key is found
 */
export async function getApiContext(): Promise<ApiContext> {
  const apiKey = await getApiKey();
  if (!apiKey) {
    throw new Error(`No API key found. Use 'elevenlabs auth login' to authenticate or set ELEVENLABS_API_KEY environment variable.`);
  }

  const config = await loadConfig();
  return { apiKey, baseUrl: getApiBaseUrl(config.residency) };
}

/**
 * Performs a raw JSON request against the Convai API.
 *
 * Agent create/update bodies deliberately bypass the SDK's generated
 * serializers: they mirror the OpenAPI spec imperfectly for recursive union
 * structures (e.g. workflow expression conditions with `llm` or `null_literal`
 * nodes) and either reject or silently strip valid configs. Sending the
 * snake_case config as-is guarantees a pulled config round-trips through push.
 */
async function convaiRequest(
  ctx: ApiContext,
  method: 'POST' | 'PATCH',
  path: string,
  body: Record<string, unknown>,
  queryParams?: Record<string, string>
): Promise<Record<string, unknown>> {
  const query = queryParams && Object.keys(queryParams).length > 0
    ? `?${new URLSearchParams(queryParams).toString()}`
    : '';

  const response = await fetch(`${ctx.baseUrl}${path}${query}`, {
    method,
    headers: {
      'xi-api-key': ctx.apiKey,
      'Content-Type': 'application/json',
      'X-Source': 'agents-cli'
    },
    body: JSON.stringify(body)
  });

  if (!response.ok) {
    const errorBody = await response.text();
    throw new Error(`${method} ${path} failed (${response.status}): ${errorBody}`);
  }

  return await response.json() as Record<string, unknown>;
}

/**
 * Creates a new agent using the ElevenLabs API.
 *
 * @param ctx - API connection context from getApiContext()
 * @param name - The name of the agent
 * @param conversationConfigDict - A dictionary for ConversationalConfig
 * @param platformSettingsDict - An optional dictionary for AgentPlatformSettings
 * @param workflow - An optional workflow configuration
 * @param tags - An optional list of tags
 * @returns Promise that resolves to the agent_id of the newly created agent
 */
export async function createAgentApi(
  ctx: ApiContext,
  name: string,
  conversationConfigDict: Record<string, unknown>,
  platformSettingsDict?: Record<string, unknown>,
  workflow?: unknown,
  tags?: string[]
): Promise<string> {
  if (typeof conversationConfigDict !== 'object' || conversationConfigDict === null) {
    throw new Error('Invalid conversation config provided');
  }

  // Clean config to remove deprecated 'tools' if 'tool_ids' exists
  const cleanedConfig = cleanConversationConfigForApi(conversationConfigDict);

  const body: Record<string, unknown> = {
    name,
    conversation_config: toSnakeCaseKeys(cleanedConfig)
  };
  if (platformSettingsDict) body.platform_settings = toSnakeCaseKeys(platformSettingsDict);
  if (workflow) body.workflow = toSnakeCaseKeys(workflow);
  if (tags) body.tags = tags;

  const response = await convaiRequest(ctx, 'POST', '/v1/convai/agents/create', body);

  return response.agent_id as string;
}

/**
 * Updates an existing agent using the ElevenLabs API.
 *
 * @param ctx - API connection context from getApiContext()
 * @param agentId - The ID of the agent to update
 * @param name - Optional new name for the agent
 * @param conversationConfigDict - Optional new dictionary for ConversationalConfig
 * @param platformSettingsDict - Optional new dictionary for AgentPlatformSettings
 * @param workflow - Optional workflow configuration
 * @param tags - Optional new list of tags
 * @returns Promise that resolves to the agent_id of the updated agent
 */
export async function updateAgentApi(
  ctx: ApiContext,
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

  const body: Record<string, unknown> = {};
  if (name !== undefined) body.name = name;
  if (cleanedConfig) body.conversation_config = toSnakeCaseKeys(cleanedConfig);
  if (platformSettingsDict) body.platform_settings = toSnakeCaseKeys(platformSettingsDict);
  if (workflow) body.workflow = toSnakeCaseKeys(workflow);
  if (tags) body.tags = tags;
  if (versionDescription !== undefined) body.version_description = versionDescription;

  const response = await convaiRequest(
    ctx,
    'PATCH',
    `/v1/convai/agents/${agentId}`,
    body,
    branchId ? { branch_id: branchId } : undefined
  );

  return {
    agentId: response.agent_id as string,
    versionId: response.version_id as string | undefined,
    branchId: response.branch_id as string | undefined
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