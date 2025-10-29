import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import PullToolsView from '../ui/PullToolsView.js';
import { pullTools } from './impl.js';

interface PullToolsOptions {
  tool?: string;
  outputDir: string;
  dryRun: boolean;
  update?: boolean;
  all?: boolean;
}

export function createPullCommand(): Command {
  return new Command('pull')
    .description('Pull tools from ElevenLabs')
    .option('--tool <tool_id>', 'Specific tool ID to pull')
    .option('--output-dir <directory>', 'Output directory for configs', 'tool_configs')
    .option('--dry-run', 'Show what would be done without making changes', false)
    .option('--update', 'Update existing items only, skip new')
    .option('--all', 'Pull all (new + existing)')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: PullToolsOptions & { ui: boolean }) => {
      try {
        if (options.ui !== false) {
          // Use Ink UI for pull
          const environments = ['prod'];
          const { waitUntilExit } = render(
            React.createElement(PullToolsView, {
              tool: options.tool,
              outputDir: options.outputDir,
              dryRun: options.dryRun,
              update: options.update,
              all: options.all,
              environments
            })
          );
          await waitUntilExit();
        } else {
          await pullTools(options);
        }
      } catch (error) {
        console.error(`Error during pull: ${error}`);
        process.exit(1);
      }
    });
}
