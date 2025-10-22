import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import WhoamiView from '../ui/WhoamiView.js';
import { getApiKey, getResidency, listEnvironments } from '../../shared/config.js';

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

          // Check if using environment variable
          if (process.env.ELEVENLABS_API_KEY) {
            const maskedKey = process.env.ELEVENLABS_API_KEY.slice(0, 8) + '...' + process.env.ELEVENLABS_API_KEY.slice(-4);
            console.log('Logged in with API key from environment variable:');
            console.log(`  prod: ${maskedKey}`);
            console.log(`Residency: ${residency}`);
          } else {
            // Load all configured environments
            const environments = await listEnvironments();

            if (environments.length > 0) {
              console.log(`Logged in to ${environments.length} environment${environments.length > 1 ? 's' : ''}:`);

              for (const env of environments) {
                const apiKey = await getApiKey(env);
                if (apiKey) {
                  const maskedKey = apiKey.slice(0, 8) + '...' + apiKey.slice(-4);
                  console.log(`  ${env}: ${maskedKey}`);
                }
              }

              console.log(`Residency: ${residency}`);
            } else {
              console.log('Not logged in');
              console.log('Use "elevenlabs auth login" to authenticate');
            }
          }
        }
      } catch (error) {
        console.error(`Error checking login status: ${error}`);
        process.exit(1);
      }
    });
}
