import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import path from 'path';
import fs from 'fs-extra';
import PushView from '../ui/PushView.js';
import { readConfig } from '../../shared/utils.js';
import { getAgentName } from './utils.js';
import { pushAgents } from './push-impl.js';

const AGENTS_CONFIG_FILE = "agents.json";

interface AgentsConfig {
  agents: Array<{
    config: string;
    id?: string;
    env?: string;
  }>;
}

interface PushOptions {
  dryRun: boolean;
}

export function createPushCommand(): Command {
  return new Command('push')
    .description('Push agents to ElevenLabs API when configs change')
    .option('--env <environment>', 'Filter agents by environment (defaults to all environments)')
    .option('--dry-run', 'Show what would be done without making changes', false)
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: PushOptions & { ui: boolean; env?: string }) => {
      try {
        if (options.ui !== false) {
          // Use new Ink UI for push
          const agentsConfigPath = path.resolve(AGENTS_CONFIG_FILE);
          if (!(await fs.pathExists(agentsConfigPath))) {
            throw new Error('agents.json not found. Run \'elevenlabs agents init\' first.');
          }

          const agentsConfig = await readConfig<AgentsConfig>(agentsConfigPath);

          // Filter agents by environment if specified
          const agentsToProcess = options.env
            ? agentsConfig.agents.filter(agent => (agent.env || 'prod') === options.env)
            : agentsConfig.agents;

          if (options.env && agentsToProcess.length === 0) {
            console.log(`No agents found for environment '${options.env}'`);
            return;
          }

          // Prepare agents for UI
          const pushAgentsData = await Promise.all(
            agentsToProcess.map(async agent => ({
              name: await getAgentName(agent.config),
              configPath: agent.config,
              status: 'pending' as const,
              agentId: agent.id,
              env: agent.env || 'prod'
            }))
          );

          const { waitUntilExit } = render(
            React.createElement(PushView, {
              agents: pushAgentsData,
              dryRun: options.dryRun
            })
          );
          await waitUntilExit();
        } else {
          // Use existing non-UI push
          await pushAgents(options.dryRun, options.env);
        }
      } catch (error) {
        console.error(`Error during push: ${error}`);
        process.exit(1);
      }
    });
}
