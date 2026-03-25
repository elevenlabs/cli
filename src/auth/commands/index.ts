import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import { createLoginCommand } from './login.js';
import { createLogoutCommand } from './logout.js';
import { createWhoamiCommand } from './whoami.js';
import { createResidencyCommand } from './residency.js';
import AuthHelpView from '../ui/AuthHelpView.js';

function printAuthHelp() {
  console.log('elevenlabs auth - Authentication commands\n');
  console.log('Usage:');
  console.log('  elevenlabs auth <command> [options]\n');
  console.log('Commands:');
  const commands = [
    { name: 'login', description: 'Login with your ElevenLabs API key' },
    { name: 'logout', description: 'Logout and remove stored API key' },
    { name: 'whoami', description: 'Show current login status' },
    { name: 'residency [location]', description: 'Set the API residency location' },
  ];
  for (const cmd of commands) {
    console.log(`  ${cmd.name.padEnd(28)}${cmd.description}`);
  }
  console.log('\nEnable interactive UI with --human-friendly flag for any command');
}

export function createAuthCommand(): Command {
  const auth = new Command('auth');
  auth.description('Authentication and configuration commands');

  // Disable default help
  auth.helpOption(false);
  auth.addHelpCommand(false);

  // Add custom help option
  auth.option('-h, --help', 'Display help information');
  auth.option('--human-friendly', 'Enable interactive terminal UI');

  // Custom action when auth command is run without subcommands
  auth.action(async (options: { humanFriendly?: boolean }) => {
    if (options.humanFriendly) {
      const { waitUntilExit } = render(
        React.createElement(AuthHelpView)
      );
      await waitUntilExit();
    } else {
      printAuthHelp();
    }
    process.exit(0);
  });

  auth.addCommand(createLoginCommand());
  auth.addCommand(createLogoutCommand());
  auth.addCommand(createWhoamiCommand());
  auth.addCommand(createResidencyCommand());

  return auth;
}
