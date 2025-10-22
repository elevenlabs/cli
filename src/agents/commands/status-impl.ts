import path from 'path';
import fs from 'fs-extra';
import { readConfig } from '../../shared/utils.js';
import { AgentConfig } from '../templates.js';
import { getAgentName } from './utils.js';

const AGENTS_CONFIG_FILE = "agents.json";

interface AgentDefinition {
  config: string;
  id?: string;
  env?: string;
}

interface AgentsConfig {
  agents: AgentDefinition[];
}

export async function showStatus(): Promise<void> {
  const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
  if (!(await fs.pathExists(agentsConfigPath))) {
    throw new Error('agents.json not found. Run \'elevenlabs agents init\' first.');
  }

  const agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);

  if (agentsConfig.agents.length === 0) {
    console.log('No agents configured');
    return;
  }

  const agentsToShow = agentsConfig.agents;

  console.log('Agent Status:');
  console.log('='.repeat(50));

  for (const agentDef of agentsToShow) {
    const agentNameCurrent = await getAgentName(agentDef.config);
    const configPath = agentDef.config;

    if (!configPath) {
      continue;
    }

    console.log(`\n${agentNameCurrent}`);
    console.log(`   Config: ${configPath}`);

    // Get agent ID from index file
    const agentId = agentDef.id || 'Not created yet';
    console.log(`   Agent ID: ${agentId}`);

    // Check config file status
    if (await fs.pathExists(configPath)) {
      try {
        await readConfig<AgentConfig>(configPath);

        // Simple status based on whether ID exists
        if (agentDef.id) {
          console.log(`   Status: Created (use push to update)`);
        } else {
          console.log(`   Status: Not pushed yet`);
        }

      } catch (error) {
        console.log(`   Status: Config error: ${error}`);
      }
    } else {
      console.log('   Status: Config file not found');
    }
  }
}
