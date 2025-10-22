import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import ListAgentsView from '../ui/ListAgentsView.js';

export function createListCommand(): Command {
  return new Command('list')
    .description('List all configured agents')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: { ui: boolean }) => {
      try {
        if (options.ui !== false) {
          // Use Ink UI for list-agents
          const { waitUntilExit } = render(
            React.createElement(ListAgentsView)
          );
          await waitUntilExit();
        } else {
          // Use the original implementation
          const { listConfiguredAgents } = await import('./utils.js');
          await listConfiguredAgents();
        }
      } catch (error) {
        console.error(`Error listing agents: ${error}`);
        process.exit(1);
      }
    });
}
