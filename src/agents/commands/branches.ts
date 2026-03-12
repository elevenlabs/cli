import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import BranchesListView from '../ui/BranchesListView.js';
import { listBranches } from './branches-impl.js';

interface BranchesListOptions {
  agent: string;
  includeArchived: boolean;
}

export function createBranchesCommand(): Command {
  const branches = new Command('branches')
    .description('Manage agent branches');

  branches
    .command('list')
    .description('List branches for an agent')
    .requiredOption('--agent <agent_id>', 'Agent ID to list branches for')
    .option('--include-archived', 'Include archived branches', false)
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: BranchesListOptions & { ui: boolean }) => {
      try {
        if (options.ui !== false) {
          const { waitUntilExit } = render(
            React.createElement(BranchesListView, {
              agent: options.agent,
              includeArchived: options.includeArchived
            })
          );
          await waitUntilExit();
        } else {
          await listBranches(options);
        }
      } catch (error) {
        console.error(`Error listing branches: ${error}`);
        process.exit(1);
      }
    });

  return branches;
}
