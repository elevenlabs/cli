import { Command } from 'commander';
import { generateWidget } from './widget-impl.js';

export function createWidgetCommand(): Command {
  return new Command('widget')
    .description('Generate HTML widget snippet for an agent')
    .argument('<name>', 'Name or ID of the agent')
    .action(async (agentId: string) => {
      try {
        await generateWidget(agentId);
      } catch (error) {
        console.error(`Error generating widget: ${error}`);
        process.exit(1);
      }
    });
}
