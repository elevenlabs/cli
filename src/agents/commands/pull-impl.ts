import path from 'path';
import fs from 'fs-extra';
import { readConfig, writeConfig, generateUniqueFilename } from '../../shared/utils.js';
import { getElevenLabsClient, listAgentsApi, getAgentApi } from '../../shared/elevenlabs-api.js';
import { AgentConfig } from '../templates.js';
import { listEnvironments } from '../../shared/config.js';
import { promptForConfirmation } from './utils.js';

const AGENTS_CONFIG_FILE = "agents.json";

interface AgentDefinition {
  config: string;
  id?: string;
}

interface AgentsConfig {
  agents: AgentDefinition[];
}

interface PullOptions {
  agent?: string;
  outputDir: string;
  dryRun: boolean;
  update?: boolean;
  all?: boolean;
}

export async function pullAgents(options: PullOptions): Promise<void> {
  const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
  const environment = 'prod';

  console.log(`Pulling from environment: ${environment}`);

  await pullAgentsFromEnvironment(options, environment, agentsConfigPath);
}

async function pullAgentsFromEnvironment(options: PullOptions, environment: string, agentsConfigPath: string): Promise<void> {
  const client = await getElevenLabsClient(environment);

  // Load existing config
  let agentsConfig: AgentsConfig;

  if (!(await fs.pathExists(agentsConfigPath))) {
    console.log(`${AGENTS_CONFIG_FILE} not found. Creating initial agents configuration...`);
    agentsConfig = { agents: [] };
    await writeConfig(agentsConfigPath, agentsConfig);
  } else {
    agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);
  }

  let agentsList: unknown[];

  if (options.agent) {
    // Pull specific agent by ID
    console.log(`Pulling agent with ID: ${options.agent}...`);
    try {
      const agentDetails = await getAgentApi(client, options.agent);
      const agentDetailsTyped = agentDetails as { agentId?: string; agent_id?: string; name: string };
      const agentId = agentDetailsTyped.agentId || agentDetailsTyped.agent_id || options.agent;
      agentsList = [{
        agentId: agentId,
        agent_id: agentId,
        name: agentDetailsTyped.name
      }];
      console.log(`Found agent: ${agentDetailsTyped.name}`);
    } catch (error) {
      throw new Error(`Failed to fetch agent with ID '${options.agent}': ${error}`);
    }
  } else {
    // Pull all agents from ElevenLabs
    console.log('Pulling all agents from ElevenLabs...');
    agentsList = await listAgentsApi(client, 30);

    if (agentsList.length === 0) {
      console.log('No agents found in your ElevenLabs workspace.');
      return;
    }

    console.log(`Found ${agentsList.length} agent(s)`);
  }

  // Build map of existing agents by ID
  const existingAgentIds = new Map(
    agentsConfig.agents
      .map(agent => [agent.id, agent])
  );

  // Track operations for summary
  const operations = { create: 0, update: 0, skip: 0 };
  type OperationItem = {
    action: 'create' | 'update' | 'skip';
    agent: { id: string; name: string };
    existingEntry?: AgentDefinition;
  };
  const itemsToProcess: OperationItem[] = [];

  // First pass: determine what will happen
  for (const agentMeta of agentsList) {
    const agentMetaTyped = agentMeta as { agentId?: string; agent_id?: string; name: string };
    const agentId = agentMetaTyped.agentId || agentMetaTyped.agent_id;
    if (!agentId) {
      console.log(`Warning: Skipping agent '${agentMetaTyped.name}' - no agent ID found`);
      continue;
    }

    let agentNameRemote = agentMetaTyped.name;
    const existingEntry = existingAgentIds.get(agentId);

    if (existingEntry) {
      // Agent with this ID already exists locally
      if (options.update || options.all) {
        // --update or --all: update existing
        itemsToProcess.push({ action: 'update', agent: { id: agentId, name: agentNameRemote }, existingEntry });
        operations.update++;
      } else {
        // Default: skip existing
        itemsToProcess.push({ action: 'skip', agent: { id: agentId, name: agentNameRemote }, existingEntry });
        operations.skip++;
      }
    } else {
      // New agent (not present locally)
      if (options.update) {
        // --update mode: skip new items (only update existing)
        itemsToProcess.push({ action: 'skip', agent: { id: agentId, name: agentNameRemote } });
        operations.skip++;
      } else {
        // Default or --all: create new items
        itemsToProcess.push({ action: 'create', agent: { id: agentId, name: agentNameRemote } });
        operations.create++;
      }
    }
  }

  // Show summary
  console.log(`\nPlan: ${operations.create} create, ${operations.update} update, ${operations.skip} skip`);

  if (operations.skip > 0 && !options.update && !options.all) {
    if (operations.create === 0) {
      console.log(`\n💡 Tip: Use --update to update existing agents or --all to pull everything`);
    } else {
      console.log(`\n💡 Tip: Use --all to also update existing agents`);
    }
  }

  // Prompt for confirmation if not --dry-run
  if (!options.dryRun && (operations.create > 0 || operations.update > 0)) {
    const confirmed = await promptForConfirmation('Proceed?');
    if (!confirmed) {
      console.log('Pull cancelled');
      return;
    }
  }

  // Second pass: execute operations
  let itemsProcessed = 0;
  for (const item of itemsToProcess) {
    const { action, agent, existingEntry } = item;

    if (action === 'skip') {
      console.log(`⊘ Skipping '${agent.name}' (already exists, use --update to overwrite)`);
      continue;
    }

    if (options.dryRun) {
      console.log(`[DRY RUN] Would ${action} agent: ${agent.name} (ID: ${agent.id})`);
      continue;
    }

    try {
      // Fetch detailed agent configuration
      console.log(`${action === 'update' ? '↻ Updating' : '+ Pulling'} config for '${agent.name}'...`);
      const agentDetails = await getAgentApi(client, agent.id);

      // Extract configuration components
      const agentDetailsTyped = agentDetails as {
        conversationConfig: Record<string, unknown>;
        conversation_config: Record<string, unknown>;
        platformSettings: Record<string, unknown>;
        platform_settings: Record<string, unknown>;
        tags: string[];
      };

      const conversationConfig = agentDetailsTyped.conversationConfig || agentDetailsTyped.conversation_config || {};
      const platformSettings = agentDetailsTyped.platformSettings || agentDetailsTyped.platform_settings || {};
      const tags = agentDetailsTyped.tags || [];

      // Create agent config structure (without agent_id - it goes in index file)
      const agentConfig: AgentConfig = {
        name: agent.name,
        conversation_config: conversationConfig as AgentConfig['conversation_config'],
        platform_settings: platformSettings,
        tags
      };

      if (action === 'update' && existingEntry) {
        // Update existing entry - overwrite the config file
        const configFilePath = path.resolve(existingEntry.config);
        await fs.ensureDir(path.dirname(configFilePath));
        await writeConfig(configFilePath, agentConfig);
        console.log(`  ✓ Updated '${agent.name}' (config: ${existingEntry.config})`);
      } else {
        // Create new entry
        const configPath = await generateUniqueFilename(options.outputDir, agent.name);
        const configFilePath = path.resolve(configPath);
        await fs.ensureDir(path.dirname(configFilePath));
        await writeConfig(configFilePath, agentConfig);

        const newAgent: AgentDefinition = {
          config: configPath,
          id: agent.id
        };

        agentsConfig.agents.push(newAgent);
        console.log(`  ✓ Added '${agent.name}' (config: ${configPath}) [${environment}]`);
      }

      itemsProcessed++;

    } catch (error) {
      console.log(`  ✗ Error ${action === 'update' ? 'updating' : 'pulling'} agent '${agent.name}': ${error}`);
      continue;
    }
  }

  // Save updated agents.json if there were changes
  if (!options.dryRun && itemsProcessed > 0) {
    await writeConfig(agentsConfigPath, agentsConfig);
    console.log(`\nUpdated ${AGENTS_CONFIG_FILE}`);
  }

  // Final summary
  if (options.dryRun) {
    console.log(`\n[DRY RUN] Would process ${operations.create + operations.update} agent(s)`);
  } else {
    console.log(`\n✓ Summary: ${operations.create} created, ${operations.update} updated, ${operations.skip} skipped`);
    if (itemsProcessed > 0) {
      console.log(`You can now edit the config files in '${options.outputDir}/' and run 'elevenlabs agents push' to update`);
    }
  }
}
