import { Command } from 'commander';
import { pushTests } from './impl.js';

interface PushTestsOptions {
  dryRun: boolean;
  configDir: string;
  ui: boolean;
  humanFriendly?: boolean;
}

export function createPushCommand(): Command {
  return new Command('push')
    .description('Push tests to ElevenLabs API')
    .option('--dry-run', 'Show what would be done without making changes', false)
    .option('--config-dir <directory>', 'Directory to scan for test configs not listed in tests.json', 'test_configs')
    .option('--no-ui', 'Disable interactive UI (default, kept for backwards compatibility)')
    .option('--human-friendly', 'Enable interactive terminal UI')
    .action(async (options: PushTestsOptions) => {
      try {
        await pushTests(undefined, options.dryRun, options.configDir);
      } catch (error) {
        console.error(`Error during push: ${error}`);
        process.exit(1);
      }
    });
}
