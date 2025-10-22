import { Command } from 'commander';
import { deleteTool, deleteAllTools } from './impl.js';

export function createDeleteCommand(): Command {
  return new Command('delete')
    .description('Delete a tool locally and from ElevenLabs')
    .argument('[tool_id]', 'ID of the tool to delete (omit with --all to delete all tools)')
    .option('--all', 'Delete all tools', false)
    .option('--env <environment>', 'Filter tools by environment (use with --all)')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (toolId: string | undefined, options: { all: boolean; env?: string; ui: boolean }) => {
      try {
        if (options.all && toolId) {
          console.error('Error: Cannot specify both tool_id and --all flag');
          process.exit(1);
        }

        if (!options.all && !toolId) {
          console.error('Error: Must specify either tool_id or --all flag');
          process.exit(1);
        }

        if (options.env && !options.all) {
          console.error('Error: --env flag can only be used with --all');
          process.exit(1);
        }

        if (options.all) {
          await deleteAllTools(options.ui, options.env);
        } else {
          await deleteTool(toolId!);
        }
      } catch (error) {
        console.error(`Error deleting tool: ${error}`);
        process.exit(1);
      }
    });
}
