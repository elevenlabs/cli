import { Command } from 'commander';
import { pullTests } from './impl.js';

interface PullTestsOptions {
  test?: string;
  outputDir: string;
  dryRun: boolean;
  update?: boolean;
  all?: boolean;
}

export function createPullCommand(): Command {
  return new Command('pull')
    .description('Pull tests from ElevenLabs')
    .option('--test <test_id>', 'Specific test ID to pull')
    .option('--output-dir <directory>', 'Output directory for configs', 'test_configs')
    .option('--dry-run', 'Show what would be done without making changes', false)
    .option('--update', 'Update existing items only, skip new')
    .option('--all', 'Pull all (new + existing)')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: PullTestsOptions & { ui: boolean }) => {
      try {
        await pullTests(options);
      } catch (error) {
        console.error(`Error during pull: ${error}`);
        process.exit(1);
      }
    });
}
