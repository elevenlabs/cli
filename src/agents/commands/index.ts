import { Command } from 'commander';
import { createInitCommand } from './init.js';
import { createAddCommand } from './add.js';
import { createListCommand } from './list.js';
import { createDeleteCommand } from './delete.js';
import { createStatusCommand } from './status.js';
import { createPushCommand } from './push.js';
import { createPullCommand } from './pull.js';
import { createTemplatesCommand } from './templates.js';
import { createWidgetCommand } from './widget.js';
import { createTestCommand } from './test.js';

export function createAgentsCommand(): Command {
  const agents = new Command('agents');
  agents.description('Manage ElevenLabs agents');

  agents.addCommand(createInitCommand());
  agents.addCommand(createAddCommand());
  agents.addCommand(createListCommand());
  agents.addCommand(createDeleteCommand());
  agents.addCommand(createStatusCommand());
  agents.addCommand(createPushCommand());
  agents.addCommand(createPullCommand());
  agents.addCommand(createTemplatesCommand());
  agents.addCommand(createWidgetCommand());
  agents.addCommand(createTestCommand());

  return agents;
}
