import path from 'path';
import fs from 'fs-extra';
import { readConfig, writeConfig } from '../../shared/utils.js';
import { getElevenLabsClient, createAgentApi, updateAgentApi } from '../../shared/elevenlabs-api.js';
import { AgentConfig } from '../templates.js';

const AGENTS_CONFIG_FILE = "agents.json";

interface AgentDefinition {
  config: string;
  id?: string;
  env?: string;
}

interface AgentsConfig {
  agents: AgentDefinition[];
}

export async function pushAgents(dryRun: boolean = false, environment?: string): Promise<void> {
  // Load agents configuration
  const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
  if (!(await fs.pathExists(agentsConfigPath))) {
    throw new Error('agents.json not found. Run \'elevenlabs agents init\' first.');
  }

  const agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);

  // Filter agents by environment if specified
  const agentsToProcess = environment
    ? agentsConfig.agents.filter(agent => (agent.env || 'prod') === environment)
    : agentsConfig.agents;

  if (environment && agentsToProcess.length === 0) {
    console.log(`No agents found for environment '${environment}'`);
    return;
  }

  if (!environment) {
    const envs = [...new Set(agentsToProcess.map(a => a.env || 'prod'))];
    console.log(`Pushing ${agentsToProcess.length} agent(s) across ${envs.length} environment(s): ${envs.join(', ')}`);
  }

  let changesMade = false;

  for (const agentDef of agentsToProcess) {
    const configPath = agentDef.config;
    const environment = agentDef.env || 'prod';

    if (!configPath) {
      console.log(`Warning: No config path found for agent`);
      continue;
    }

    // Check if config file exists
    if (!(await fs.pathExists(configPath))) {
      console.log(`Warning: Config file not found: ${configPath}`);
      continue;
    }

    // Load agent config
    let agentConfig: AgentConfig;
    try {
      agentConfig = await readConfig<AgentConfig>(configPath);
    } catch (error) {
      console.log(`Error reading config for ${configPath}: ${error}`);
      continue;
    }

    const agentDefName = agentConfig.name || 'Unnamed Agent';

    // Get agent ID from index file
    const agentId = agentDef.id;

    // Always push (force override)
    console.log(`${agentDefName} [${environment}]: Will push (force override)`);

    if (dryRun) {
      console.log(`[DRY RUN] Would update agent: ${agentDefName} [${environment}]`);
      continue;
    }

    // Initialize ElevenLabs client for this agent's environment
    let client;
    try {
      client = await getElevenLabsClient(environment);
    } catch (error) {
      console.log(`Error: ${error}`);
      console.log(`Skipping agent ${agentDefName} - environment '${environment}' not configured`);
      continue;
    }

    // Perform API operation
    try {
      // Extract config components
      const conversationConfig = agentConfig.conversation_config || {};
      const platformSettings = agentConfig.platform_settings;
      const tags = agentConfig.tags || [];

      const agentDisplayName = agentConfig.name;

      if (!agentId) {
        // Create new agent
        const newAgentId = await createAgentApi(
          client,
          agentDisplayName,
          conversationConfig,
          platformSettings,
          tags
        );
        console.log(`Created agent ${agentDefName} (ID: ${newAgentId}) [${environment}]`);

        // Store agent ID in index file
        agentDef.id = newAgentId;
        changesMade = true;
      } else {
        // Update existing agent
        await updateAgentApi(
          client,
          agentId,
          agentDisplayName,
          conversationConfig,
          platformSettings,
          tags
        );
        console.log(`Updated agent ${agentDefName} (ID: ${agentId}) [${environment}]`);
      }

      changesMade = true;

    } catch (error) {
      console.log(`Error processing ${agentDefName}: ${error}`);
    }
  }

  // Save updated agents.json if there were changes
  if (changesMade) {
    await writeConfig(agentsConfigPath, agentsConfig);
  }
}
