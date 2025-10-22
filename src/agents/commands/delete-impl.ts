import path from 'path';
import fs from 'fs-extra';
import { readConfig, writeConfig } from '../../shared/utils.js';
import { getElevenLabsClient, deleteAgentApi } from '../../shared/elevenlabs-api.js';
import { getAgentName, promptForConfirmation } from './utils.js';

const AGENTS_CONFIG_FILE = "agents.json";

interface AgentDefinition {
  config: string;
  id?: string;
  env?: string;
}

interface AgentsConfig {
  agents: AgentDefinition[];
}

export async function deleteAgent(agentId: string): Promise<void> {
  // Load agents configuration
  const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
  if (!(await fs.pathExists(agentsConfigPath))) {
    throw new Error('agents.json not found. Run \'elevenlabs agents init\' first.');
  }

  const agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);

  // Find the agent by ID
  const agentIndex = agentsConfig.agents.findIndex(agent => agent.id === agentId);

  if (agentIndex === -1) {
    throw new Error(`Agent with ID '${agentId}' not found in local configuration`);
  }

  const agentDef = agentsConfig.agents[agentIndex];
  const agentName = await getAgentName(agentDef.config);
  const configPath = agentDef.config;
  const environment = agentDef.env || 'prod';

  console.log(`Deleting agent '${agentName}' (ID: ${agentId}) [${environment}]...`);

  // Delete from ElevenLabs (globally)
  console.log('Deleting from ElevenLabs...');
  const client = await getElevenLabsClient(environment);

  try {
    await deleteAgentApi(client, agentId);
    console.log('✓ Successfully deleted from ElevenLabs');
  } catch (error) {
    console.error(`Warning: Failed to delete from ElevenLabs: ${error}`);
    console.log('Continuing with local deletion...');
  }

  // Remove from local agents.json
  agentsConfig.agents.splice(agentIndex, 1);
  await writeConfig(agentsConfigPath, agentsConfig);
  console.log(`✓ Removed '${agentName}' from ${AGENTS_CONFIG_FILE}`);

  // Remove config file
  if (configPath && await fs.pathExists(configPath)) {
    await fs.remove(configPath);
    console.log(`✓ Deleted config file: ${configPath}`);
  }

  console.log(`\n✓ Successfully deleted agent '${agentName}'`);
}

export async function deleteAllAgents(ui: boolean = true, env?: string): Promise<void> {
  // Load agents configuration
  const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
  if (!(await fs.pathExists(agentsConfigPath))) {
    throw new Error('agents.json not found. Run \'elevenlabs agents init\' first.');
  }

  const agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);

  if (agentsConfig.agents.length === 0) {
    console.log('No agents found to delete');
    return;
  }

  // Filter agents by environment if specified
  const agentsToDelete = env
    ? agentsConfig.agents.filter(agent => (agent.env || 'prod') === env)
    : agentsConfig.agents;

  if (agentsToDelete.length === 0) {
    console.log(env ? `No agents found in environment '${env}'` : 'No agents found to delete');
    return;
  }

  // Show what will be deleted
  const envInfo = env ? ` in environment '${env}'` : '';
  console.log(`\nFound ${agentsToDelete.length} agent(s) to delete${envInfo}:`);
  for (let i = 0; i < agentsToDelete.length; i++) {
    const agent = agentsToDelete[i];
    const agentName = await getAgentName(agent.config);
    const agentEnv = agent.env || 'prod';
    console.log(`  ${i + 1}. ${agentName} (${agent.id}) [${agentEnv}]`);
  }

  // Confirm deletion (skip if --no-ui)
  if (ui) {
    const warningMsg = env
      ? `\nWARNING: This will delete ${agentsToDelete.length} agent(s) from environment '${env}' in both local configuration and ElevenLabs.`
      : '\nWARNING: This will delete ALL agents from both local configuration and ElevenLabs.';
    console.log(warningMsg);
    const confirmed = await promptForConfirmation('Are you sure you want to delete these agents?');

    if (!confirmed) {
      console.log('Deletion cancelled');
      return;
    }
  }

  console.log('\nDeleting agents...\n');

  let successCount = 0;
  let failCount = 0;
  const deletedIds = new Set<string>();

  // Delete each agent
  for (const agentDef of agentsToDelete) {
    try {
      const agentName = await getAgentName(agentDef.config);
      const environment = agentDef.env || 'prod';
      console.log(`Deleting '${agentName}' (${agentDef.id}) [${environment}]...`);

      // Delete from ElevenLabs
      if (agentDef.id) {
        try {
          const client = await getElevenLabsClient(environment);
          await deleteAgentApi(client, agentDef.id);
          console.log(`  ✓ Deleted from ElevenLabs`);
        } catch (error) {
          console.error(`  Warning: Failed to delete from ElevenLabs: ${error}`);
        }
      } else {
        console.log(`  Warning: No agent ID found, skipping ElevenLabs deletion`);
      }

      // Remove config file
      if (agentDef.config && await fs.pathExists(agentDef.config)) {
        await fs.remove(agentDef.config);
        console.log(`  ✓ Deleted config file: ${agentDef.config}`);
      }

      if (agentDef.id) {
        deletedIds.add(agentDef.id);
      }
      successCount++;
    } catch (error) {
      const agentName = await getAgentName(agentDef.config);
      console.error(`  Failed to delete '${agentName}': ${error}`);
      failCount++;
    }
  }

  // Remove deleted agents from config (only the ones that were successfully deleted)
  agentsConfig.agents = agentsConfig.agents.filter(agent => !deletedIds.has(agent.id || ''));
  await writeConfig(agentsConfigPath, agentsConfig);
  console.log(`\n✓ Updated ${AGENTS_CONFIG_FILE}`);

  // Summary
  console.log(`\n✓ Deletion complete: ${successCount} succeeded, ${failCount} failed`);
}
