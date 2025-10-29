import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import LogoutView from '../ui/LogoutView.js';
import { removeApiKey, isLoggedIn } from '../../shared/config.js';

export function createLogoutCommand(): Command {
  return new Command('logout')
    .description('Logout and remove stored API key')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: { ui: boolean }) => {
      try {
        const environment = 'prod';

        if (options.ui !== false) {
          // Use Ink UI for logout
          const { waitUntilExit } = render(
            React.createElement(LogoutView, {})
          );
          await waitUntilExit();
        } else {
          // Fallback to text-based logout
          const loggedIn = await isLoggedIn(environment);
          if (!loggedIn) {
            console.log(`You are not logged in to environment '${environment}'`);
            return;
          }

          await removeApiKey(environment);
          console.log(`Logged out successfully from environment '${environment}'. API key removed.`);
        }
      } catch (error) {
        console.error(`Error during logout: ${error}`);
        process.exit(1);
      }
    });
}
