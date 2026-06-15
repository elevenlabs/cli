import { ElevenLabsClient } from '@elevenlabs/elevenlabs-js';
import { ElevenLabs } from '@elevenlabs/elevenlabs-js';
import { getAgentApi, cleanConversationConfigForApi } from './elevenlabs-api.js';
import { toSnakeCaseKeys } from './utils.js';

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return (
    typeof value === "object" &&
    value !== null &&
    Object.prototype.toString.call(value) === "[object Object]"
  );
}

/**
 * Collects the key paths from `expected` that are missing in `actual`.
 *
 * Presence-only subset check: plain objects are traversed recursively, while
 * arrays and scalar values are treated as leaves (a present key counts, values
 * are not compared). Extra keys in `actual` are ignored, since the API merges
 * updates and fills defaults.
 *
 * @param expected - The configuration that was pushed
 * @param actual - The configuration the API reports as persisted
 * @param basePath - Path prefix used during recursion
 * @returns Dotted paths of expected keys that did not persist
 */
export function findMissingPaths(expected: unknown, actual: unknown, basePath: string = ''): string[] {
  if (!isPlainObject(expected)) {
    return [];
  }

  const missing: string[] = [];

  for (const [key, expectedValue] of Object.entries(expected)) {
    const path = basePath ? `${basePath}.${key}` : key;
    const actualValue = isPlainObject(actual) ? actual[key] : undefined;

    if (actualValue === undefined) {
      missing.push(path);
    } else if (isPlainObject(expectedValue)) {
      missing.push(...findMissingPaths(expectedValue, actualValue, path));
    }
  }

  return missing;
}

function reportMissingFields(label: string, missing: string[], pullCommand: string): void {
  if (missing.length === 0) {
    return;
  }

  const shown = missing.slice(0, 10);
  console.log(`  ⚠ ${label}: ${missing.length} field(s) in the local config were not persisted by the API:`);
  for (const path of shown) {
    console.log(`      - ${path}`);
  }
  if (missing.length > shown.length) {
    console.log(`      ... and ${missing.length - shown.length} more`);
  }
  console.log(`    These fields may not be supported by the CLI's bundled SDK. Run '${pullCommand}' to inspect the live config.`);
}

export interface AgentPushBlocks {
  conversation_config?: Record<string, unknown>;
  platform_settings?: Record<string, unknown>;
  workflow?: unknown;
}

/**
 * Reads an agent back after a push and warns about any locally-specified
 * field the API did not persist. The SDK serializes requests with
 * unrecognizedObjectKeys: "strip", so fields the bundled SDK does not model
 * are silently dropped before they reach the API; without a read-back the
 * CLI would report success regardless. Never fails the push: verification
 * problems are reported as warnings only.
 */
export async function verifyAgentPush(
  client: ElevenLabsClient,
  label: string,
  agentId: string,
  pushed: AgentPushBlocks,
  branchId?: string
): Promise<void> {
  let live: unknown;
  try {
    live = await getAgentApi(client, agentId, branchId);
  } catch (error) {
    console.log(`  ⚠ ${label}: could not verify the pushed config: ${error}`);
    return;
  }

  const expected: Record<string, unknown> = {};
  if (pushed.conversation_config) {
    // Mirror the push payload: the deprecated 'tools' field is removed when
    // 'tool_ids' is present, so its absence live is intentional
    expected.conversation_config = cleanConversationConfigForApi(pushed.conversation_config);
  }
  if (pushed.platform_settings) {
    expected.platform_settings = pushed.platform_settings;
  }
  if (pushed.workflow !== undefined && pushed.workflow !== null) {
    expected.workflow = pushed.workflow;
  }

  // Normalize both sides the same way (getAgentApi already snake_cases the
  // live config), so only genuinely dropped fields are reported
  const missing = findMissingPaths(toSnakeCaseKeys(expected), live);
  reportMissingFields(label, missing, 'elevenlabs agents pull');
}

/**
 * Verifies a pushed tool config against the config returned by the API.
 * Tool create/update responses already contain the persisted tool config,
 * so no extra API call is needed.
 */
export function verifyToolPush(
  label: string,
  pushedConfig: Record<string, unknown>,
  response: ElevenLabs.ToolResponseModel | undefined
): void {
  const liveConfig = response?.toolConfig;
  if (!liveConfig) {
    console.log(`  ⚠ ${label}: could not verify the pushed config (no tool config in the API response)`);
    return;
  }

  const missing = findMissingPaths(toSnakeCaseKeys(pushedConfig), toSnakeCaseKeys(liveConfig));
  reportMissingFields(label, missing, 'elevenlabs tools pull');
}
