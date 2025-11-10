import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import WhoamiView from '../ui/WhoamiView.js';
import { getApiKey, getResidency } from '../../shared/config.js';

export function createWhoamiCommand(): Command {
  return new Command('whoami')
    .description('Show current login status')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: { ui: boolean }) => {
      try {
        if (options.ui !== false) {
          // Use Ink UI for whoami
          const { waitUntilExit } = render(
            React.createElement(WhoamiView)
          );
          await waitUntilExit();
        } else {
          // Fallback to text-based output
          const residency = await getResidency();
          const apiKey = await getApiKey();

          if (apiKey) {
            const maskedKey = apiKey.slice(0, 8) + '...' + apiKey.slice(-4);
            const source = process.env.ELEVENLABS_API_KEY ? ' (from environment variable)' : '';
            console.log(`Logged in: ${maskedKey}${source}`);
            console.log(`Residency: ${residency}`);
          } else {
            console.log('Not logged in');
            console.log('Use "elevenlabs auth login" to authenticate');
          }
        }
      } catch (error) {
        console.error(`Error checking login status: ${error}`);
        process.exit(1);
      }
    });
}
