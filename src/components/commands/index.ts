import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import { createAddCommand } from './add.js';
import ComponentsHelpView from '../ui/ComponentsHelpView.js';

export function createComponentsCommand(): Command {
  const components = new Command('components');
  components.description('Import components from the ElevenLabs UI registry (https://ui.elevenlabs.io)');

  // Disable default help
  components.helpOption(false);
  components.addHelpCommand(false);

  // Add custom help option
  components.option('-h, --help', 'Display help information');

  // Custom action when components command is run without subcommands
  components.action(async () => {
    const { waitUntilExit } = render(
      React.createElement(ComponentsHelpView)
    );
    await waitUntilExit();
    process.exit(0);
  });

  components.addCommand(createAddCommand());

  return components;
}
