import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import StatusView from '../ui/StatusView.js';
import { showStatus } from './status-impl.js';

interface StatusOptions {
  agent?: string;
}

export function createStatusCommand(): Command {
  return new Command('status')
    .description('Show the status of agents')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: StatusOptions & { ui: boolean }) => {
      try {
        if (options.ui !== false) {
          // Use Ink UI for status display
          const { waitUntilExit } = render(
            React.createElement(StatusView, {})
          );
          await waitUntilExit();
        } else {
          await showStatus();
        }
      } catch (error) {
        console.error(`Error showing status: ${error}`);
        process.exit(1);
      }
    });
}
