import path from 'path';
import fs from 'fs-extra';
import { readConfig } from '../../shared/utils.js';
import { getResidency } from '../../shared/config.js';
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

export async function generateWidget(agentId: string): Promise<void> {
  // Load agents configuration
  const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
  if (!(await fs.pathExists(agentsConfigPath))) {
    throw new Error('agents.json not found. Run \'elevenlabs agents init\' first.');
  }

  // Check if agent exists in config
  const agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);
  const agentDef = agentsConfig.agents.find(agent => agent.id === agentId);

  if (!agentDef) {
    throw new Error(`Agent with ID '${agentId}' not found in configuration`);
  }

  const residency = await getResidency();

  // Generate HTML widget snippet with server-location attribute
  let htmlSnippet = `<elevenlabs-convai agent-id="${agentId}"`;

  // Add server-location attribute for isolated regions
  if (residency !== 'global' && residency !== 'us') {
    htmlSnippet += ` server-location="${residency}"`;
  }

  htmlSnippet += `></elevenlabs-convai>
<script src="https://unpkg.com/@elevenlabs/convai-widget-embed" async type="text/javascript"></script>`;

  const agentName = await getAgentName(agentDef.config);
  console.log(`HTML Widget for agent '${agentName}' (residency: ${residency}):`);
  console.log('='.repeat(60));
  console.log(htmlSnippet);
  console.log('='.repeat(60));
  console.log(`Agent ID: ${agentId}`);
}
