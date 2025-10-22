import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import PushToolsView from '../ui/PushToolsView.js';
import { pushTools } from './impl.js';

export function createPushCommand(): Command {
  return new Command('push')
    .description('Push tools to ElevenLabs API')
    .option('--env <environment>', 'Filter tools by environment')
    .option('--dry-run', 'Show what would be done without making changes', false)
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: { dryRun: boolean; env?: string; ui: boolean }) => {
      try {
        if (options.ui !== false) {
          // PushToolsView requires tools array to be prepared
          // For now, use the non-UI implementation
          await pushTools(undefined, options.dryRun, options.env);
        } else {
          await pushTools(undefined, options.dryRun, options.env);
        }
      } catch (error) {
        console.error(`Error during push: ${error}`);
        process.exit(1);
      }
    });
}
