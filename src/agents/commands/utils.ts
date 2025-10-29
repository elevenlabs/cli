import path from 'path';
import fs from 'fs-extra';
import { readConfig } from '../../shared/utils.js';

const AGENTS_CONFIG_FILE = "agents.json";

interface AgentsConfig {
  agents: Array<{
    config: string;
    id?: string;
  }>;
}

export async function getAgentName(configPath: string): Promise<string> {
  try {
    const fullPath = path.resolve(configPath);
    if (await fs.pathExists(fullPath)) {
      const config = await readConfig<{ name?: string }>(fullPath);
      return config.name || 'Unnamed Agent';
    }
    return 'Unknown Agent';
  } catch {
    return 'Unknown Agent';
  }
}

export async function listConfiguredAgents(): Promise<void> {
  const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
  if (!(await fs.pathExists(agentsConfigPath))) {
    throw new Error('agents.json not found. Run \'elevenlabs agents init\' first.');
  }

  const agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);

  if (agentsConfig.agents.length === 0) {
    console.log('No agents configured');
    return;
  }

  console.log('Configured Agents:');
  console.log('='.repeat(50));

  for (let i = 0; i < agentsConfig.agents.length; i++) {
    const agentDef = agentsConfig.agents[i];
    const agentName = await getAgentName(agentDef.config);
    const agentId = agentDef.id || 'No ID';
    console.log(`${i + 1}. ${agentName}`);
    console.log(`   ID: ${agentId}`);
    const configPath = agentDef.config || 'No config path';
    console.log(`   Config: ${configPath}`);
    console.log();
  }
}

export async function promptForConfirmation(message: string): Promise<boolean> {
  const readline = await import('readline');
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });

  return new Promise<boolean>((resolve) => {
    rl.question(`${message} (y/N): `, (answer) => {
      rl.close();
      resolve(answer.toLowerCase() === 'y' || answer.toLowerCase() === 'yes');
    });
  });
}
