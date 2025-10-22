import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import PullView from '../ui/PullView.js';
import { pullAgents } from './pull-impl.js';

interface PullOptions {
  agent?: string;
  outputDir: string;
  dryRun: boolean;
  update?: boolean;
  all?: boolean;
  env?: string;
}

export function createPullCommand(): Command {
  return new Command('pull')
    .description('Pull agents from ElevenLabs')
    .option('--agent <agent_id>', 'Specific agent ID to pull')
    .option('--output-dir <directory>', 'Output directory for configs', 'agent_configs')
    .option('--dry-run', 'Show what would be done without making changes', false)
    .option('--update', 'Update existing items only, skip new')
    .option('--all', 'Pull all (new + existing)')
    .option('--env <environment>', 'Environment to pull from (defaults to all)')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: PullOptions & { ui: boolean }) => {
      try {
        if (options.ui !== false) {
          // Use Ink UI for pull
          const environments = options.env ? [options.env] : ['prod']; // Default to prod if no env specified
          const { waitUntilExit } = render(
            React.createElement(PullView, {
              agent: options.agent,
              outputDir: options.outputDir,
              dryRun: options.dryRun,
              update: options.update,
              all: options.all,
              environments
            })
          );
          await waitUntilExit();
        } else {
          await pullAgents(options);
        }
      } catch (error) {
        console.error(`Error during pull: ${error}`);
        process.exit(1);
      }
    });
}
