import { Command } from 'commander';
import { createAddCommand } from './add.js';
import { createDeleteCommand } from './delete.js';
import { createPushCommand } from './push.js';
import { createPullCommand } from './pull.js';

export function createToolsCommand(): Command {
  const tools = new Command('tools');
  tools.description('Manage ElevenLabs tools');

  tools.addCommand(createAddCommand());
  tools.addCommand(createDeleteCommand());
  tools.addCommand(createPushCommand());
  tools.addCommand(createPullCommand());

  return tools;
}
