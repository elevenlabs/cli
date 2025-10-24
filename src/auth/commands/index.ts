import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import { createLoginCommand } from './login.js';
import { createLogoutCommand } from './logout.js';
import { createWhoamiCommand } from './whoami.js';
import { createResidencyCommand } from './residency.js';
import AuthHelpView from '../ui/AuthHelpView.js';

export function createAuthCommand(): Command {
  const auth = new Command('auth');
  auth.description('Authentication and configuration commands');

  // Disable default help
  auth.helpOption(false);
  auth.addHelpCommand(false);

  // Add custom help option
  auth.option('-h, --help', 'Display help information');

  // Custom action when auth command is run without subcommands
  auth.action(async (options) => {
    const { waitUntilExit } = render(
      React.createElement(AuthHelpView)
    );
    await waitUntilExit();
    process.exit(0);
  });

  auth.addCommand(createLoginCommand());
  auth.addCommand(createLogoutCommand());
  auth.addCommand(createWhoamiCommand());
  auth.addCommand(createResidencyCommand());

  return auth;
}
