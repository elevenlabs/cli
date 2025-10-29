import { Command } from 'commander';
import { addTool } from './impl.js';

export function createAddCommand(): Command {
  return new Command('add')
    .description('Add a new tool')
    .argument('<name>', 'Name of the tool to create')
    .option('--type <type>', 'Tool type: webhook or client', 'webhook')
    .option('--config-path <path>', 'Custom config path (optional)')
    .action(async (name: string, options: { type: string; configPath?: string }) => {
      try {
        if (options.type !== 'webhook' && options.type !== 'client') {
          console.error('Error: --type must be either "webhook" or "client"');
          process.exit(1);
        }

        await addTool(name, options.type as 'webhook' | 'client', options.configPath);
      } catch (error) {
        console.error(`Error creating tool: ${error}`);
        process.exit(1);
      }
    });
}
