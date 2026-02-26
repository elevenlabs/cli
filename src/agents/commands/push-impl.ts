import path from 'path';
import fs from 'fs-extra';
import { readConfig, writeConfig } from '../../shared/utils.js';
import { getElevenLabsClient, createAgentApi, updateAgentApi } from '../../shared/elevenlabs-api.js';
import { AgentConfig } from '../templates.js';

const AGENTS_CONFIG_FILE = "agents.json";

interface AgentDefinition {
  config: string;
  id?: string;
  branch_id?: string;
  version_id?: string;
}

interface AgentsConfig {
  agents: AgentDefinition[];
}

export async function pushAgents(dryRun: boolean = false, agentId?: string, versionDescription?: string): Promise<void> {
  // Load agents configuration
  const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
  if (!(await fs.pathExists(agentsConfigPath))) {
    throw new Error('agents.json not found. Run \'elevenlabs agents init\' first.');
  }

  const agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);

  let agentsToProcess = agentsConfig.agents;
  // Filter to specific agent if agentId is provided
  if (agentId) {
    agentsToProcess = agentsToProcess.filter(agent => agent.id === agentId);
    if (agentsToProcess.length === 0) {
      throw new Error(`Agent with ID ${agentId} not found in agents.json`);
    }
  }

  console.log(`Pushing ${agentsToProcess.length} agent(s) to ElevenLabs...`);

  let changesMade = false;

  for (const agentDef of agentsToProcess) {
    const configPath = agentDef.config;

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
    console.log(`${agentDefName}: Will push (force override)`);

    if (dryRun) {
      console.log(`[DRY RUN] Would update agent: ${agentDefName}`);
      continue;
    }

    // Initialize ElevenLabs client
    let client;
    try {
      client = await getElevenLabsClient();
    } catch (error) {
      console.log(`Error: ${error}`);
      console.log(`Skipping agent ${agentDefName} - not configured`);
      continue;
    }

    // Perform API operation
    try {
      // Extract config components
      const conversationConfig = agentConfig.conversation_config || {};
      const platformSettings = agentConfig.platform_settings;
      const workflow = agentConfig.workflow;
      const tags = agentConfig.tags || [];

      const agentDisplayName = agentConfig.name;

      if (!agentId) {
        // Create new agent
        const newAgentId = await createAgentApi(
          client,
          agentDisplayName,
          conversationConfig,
          platformSettings,
          workflow,
          tags
        );
        console.log(`Created agent ${agentDefName} (ID: ${newAgentId})`);

        // Store agent ID in index file
        agentDef.id = newAgentId;
        changesMade = true;
      } else {
        // Update existing agent
        const result = await updateAgentApi(
          client,
          agentId,
          agentDisplayName,
          conversationConfig,
          platformSettings,
          workflow,
          tags,
          versionDescription
        );
        console.log(`Updated agent ${agentDefName} (ID: ${agentId})`);

        // Update version/branch info
        if (result.versionId) agentDef.version_id = result.versionId;
        if (result.branchId) agentDef.branch_id = result.branchId;
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
