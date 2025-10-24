import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import { createAddCommand } from './add.js';
import { createDeleteCommand } from './delete.js';
import { createPushCommand } from './push.js';
import { createPullCommand } from './pull.js';
import TestsHelpView from '../ui/TestsHelpView.js';

export function createTestsCommand(): Command {
  const tests = new Command('tests');
  tests.description('Manage ElevenLabs tests');

  // Disable default help
  tests.helpOption(false);
  tests.addHelpCommand(false);

  // Add custom help option
  tests.option('-h, --help', 'Display help information');

  // Custom action when tests command is run without subcommands
  tests.action(async (options) => {
    const { waitUntilExit } = render(
      React.createElement(TestsHelpView)
    );
    await waitUntilExit();
    process.exit(0);
  });

  tests.addCommand(createAddCommand());
  tests.addCommand(createDeleteCommand());
  tests.addCommand(createPushCommand());
  tests.addCommand(createPullCommand());

  return tests;
}
