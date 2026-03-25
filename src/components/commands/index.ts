import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import { createAddCommand } from './add.js';
import ComponentsHelpView from '../ui/ComponentsHelpView.js';

function printComponentsHelp() {
  console.log('elevenlabs components - UI component management\n');
  console.log('Usage:');
  console.log('  elevenlabs components <command> [options]\n');
  console.log('Commands:');
  const commands = [
    { name: 'add [name]', description: 'Add a component from the ElevenLabs UI registry' },
  ];
  for (const cmd of commands) {
    console.log(`  ${cmd.name.padEnd(28)}${cmd.description}`);
  }
  console.log('\nComponents are sourced from https://ui.elevenlabs.io');
}

export function createComponentsCommand(): Command {
  const components = new Command('components');
  components.description('Import components from the ElevenLabs UI registry (https://ui.elevenlabs.io)');

  // Disable default help
  components.helpOption(false);
  components.addHelpCommand(false);

  // Add custom help option
  components.option('-h, --help', 'Display help information');
  components.option('--human-friendly', 'Enable interactive terminal UI');

  // Custom action when components command is run without subcommands
  components.action(async () => {
    if (process.argv.includes('--human-friendly')) {
      const { waitUntilExit } = render(
        React.createElement(ComponentsHelpView)
      );
      await waitUntilExit();
    } else {
      printComponentsHelp();
    }
    process.exit(0);
  });

  components.addCommand(createAddCommand());

  return components;
}
