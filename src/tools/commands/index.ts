import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import { createAddCommand } from './add.js';
import { createDeleteCommand } from './delete.js';
import { createPushCommand } from './push.js';
import { createPullCommand } from './pull.js';
import ToolsHelpView from '../ui/ToolsHelpView.js';

export function createToolsCommand(): Command {
  const tools = new Command('tools');
  tools.description('Manage ElevenLabs tools');

  // Disable default help
  tools.helpOption(false);
  tools.addHelpCommand(false);

  // Add custom help option
  tools.option('-h, --help', 'Display help information');

  // Custom action when tools command is run without subcommands
  tools.action(async () => {
    const { waitUntilExit } = render(
      React.createElement(ToolsHelpView)
    );
    await waitUntilExit();
    process.exit(0);
  });

  tools.addCommand(createAddCommand());
  tools.addCommand(createDeleteCommand());
  tools.addCommand(createPushCommand());
  tools.addCommand(createPullCommand());

  return tools;
}
