import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
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
import AgentsHelpView from '../ui/AgentsHelpView.js';

export function createAgentsCommand(): Command {
  const agents = new Command('agents');
  agents.description('Manage ElevenLabs agents');

  // Disable default help
  agents.helpOption(false);
  agents.addHelpCommand(false);

  // Add custom help option
  agents.option('-h, --help', 'Display help information');

  // Custom action when agents command is run without subcommands
  agents.action(async () => {
    const { waitUntilExit } = render(
      React.createElement(AgentsHelpView)
    );
    await waitUntilExit();
    process.exit(0);
  });

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
