import { Command } from 'commander';
import { pushTools } from './impl.js';

export function createPushCommand(): Command {
  return new Command('push')
    .description('Push tools to ElevenLabs API')
    .option('--dry-run', 'Show what would be done without making changes', false)
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: { dryRun: boolean; ui: boolean }) => {
      try {
        if (options.ui !== false) {
          // PushToolsView requires tools array to be prepared
          // For now, use the non-UI implementation
          await pushTools(undefined, options.dryRun);
        } else {
          await pushTools(undefined, options.dryRun);
        }
      } catch (error) {
        console.error(`Error during push: ${error}`);
        process.exit(1);
      }
    });
}
