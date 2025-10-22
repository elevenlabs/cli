import { Command } from 'commander';
import { deleteTest, deleteAllTests } from './impl.js';

export function createDeleteCommand(): Command {
  return new Command('delete')
    .description('Delete a test locally and from ElevenLabs')
    .argument('[test_id]', 'ID of the test to delete (omit with --all to delete all tests)')
    .option('--all', 'Delete all tests', false)
    .option('--env <environment>', 'Filter tests by environment (use with --all)')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (testId: string | undefined, options: { all: boolean; env?: string; ui: boolean }) => {
      try {
        if (options.all && testId) {
          console.error('Error: Cannot specify both test_id and --all flag');
          process.exit(1);
        }

        if (!options.all && !testId) {
          console.error('Error: Must specify either test_id or --all flag');
          process.exit(1);
        }

        if (options.env && !options.all) {
          console.error('Error: --env flag can only be used with --all');
          process.exit(1);
        }

        if (options.all) {
          await deleteAllTests(options.ui, options.env);
        } else {
          await deleteTest(testId!);
        }
      } catch (error) {
        console.error(`Error deleting test: ${error}`);
        process.exit(1);
      }
    });
}
