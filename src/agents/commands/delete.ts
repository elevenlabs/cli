import { Command } from 'commander';

// These will be imported from a shared utils or implementation file
import { deleteAgent, deleteAllAgents } from './delete-impl.js';

export function createDeleteCommand(): Command {
  return new Command('delete')
    .description('Delete an agent locally and from ElevenLabs')
    .argument('[agent_id]', 'ID of the agent to delete (omit with --all to delete all agents)')
    .option('--all', 'Delete all agents', false)
    .option('--env <environment>', 'Filter agents by environment (use with --all)')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (agentId: string | undefined, options: { all: boolean; env?: string; ui: boolean }) => {
      try {
        if (options.all && agentId) {
          console.error('Error: Cannot specify both agent_id and --all flag');
          process.exit(1);
        }

        if (!options.all && !agentId) {
          console.error('Error: Must specify either agent_id or --all flag');
          process.exit(1);
        }

        if (options.env && !options.all) {
          console.error('Error: --env flag can only be used with --all');
          process.exit(1);
        }

        if (options.all) {
          await deleteAllAgents(options.ui, options.env);
        } else {
          await deleteAgent(agentId!);
        }
      } catch (error) {
        console.error(`Error deleting agent: ${error}`);
        process.exit(1);
      }
    });
}
