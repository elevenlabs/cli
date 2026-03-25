import { Command } from 'commander';
import { pushTests } from './impl.js';

export function createPushCommand(): Command {
  return new Command('push')
    .description('Push tests to ElevenLabs API')
    .option('--dry-run', 'Show what would be done without making changes', false)
    .option('--no-ui', 'Disable interactive UI (default, kept for backwards compatibility)')
    .option('--human-friendly', 'Enable interactive terminal UI')
    .action(async (options: { dryRun: boolean; ui: boolean; humanFriendly?: boolean }) => {
      try {
        await pushTests(undefined, options.dryRun);
      } catch (error) {
        console.error(`Error during push: ${error}`);
        process.exit(1);
      }
    });
}
