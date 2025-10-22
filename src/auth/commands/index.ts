import { Command } from 'commander';
import { createLoginCommand } from './login.js';
import { createLogoutCommand } from './logout.js';
import { createWhoamiCommand } from './whoami.js';
import { createResidencyCommand } from './residency.js';

export function createAuthCommand(): Command {
  const auth = new Command('auth');
  auth.description('Authentication and configuration commands');

  auth.addCommand(createLoginCommand());
  auth.addCommand(createLogoutCommand());
  auth.addCommand(createWhoamiCommand());
  auth.addCommand(createResidencyCommand());

  return auth;
}
