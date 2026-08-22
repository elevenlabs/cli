import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import LoginView from '../ui/LoginView.js';
import { setApiKey, loadConfig } from '../../shared/config.js';
import { getApiBaseUrl } from '../../shared/elevenlabs-api.js';
import { listAgentsApi } from '../../shared/elevenlabs-api.js';
import { ElevenLabsClient } from '@elevenlabs/elevenlabs-js';
import { formatApiKeyValidationError } from '../api-key-validation.js';

export function createLoginCommand(): Command {
  return new Command('login')
    .description('Login with your ElevenLabs API key')
    .option('--no-ui', 'Disable interactive UI (default, kept for backwards compatibility)')
    .option('--human-friendly', 'Enable interactive terminal UI')
    .action(async (options: { ui: boolean; humanFriendly?: boolean }) => {
      try {
        if (options.humanFriendly) {
          // Use Ink UI for login
          const { waitUntilExit } = render(
            React.createElement(LoginView, {})
          );
          await waitUntilExit();
        } else {
          // Fallback to text-based login
          const { read } = await import('read');

          console.log('Logging in to ElevenLabs...');
          const apiKey = await read({
            prompt: 'Enter your ElevenLabs API key: ',
            silent: true,
            replace: '*'
          });

          if (!apiKey || apiKey.trim() === '') {
            console.error('API key is required');
            process.exit(1);
          }

          // Test the API key by making a simple request
          // Create client directly with the provided API key for validation
          const config = await loadConfig();
          const baseURL = getApiBaseUrl(config.residency);
          const testClient = new ElevenLabsClient({
            apiKey: apiKey.trim(),
            baseUrl: baseURL,
            headers: {
              'X-Source': 'agents-cli'
            }
          });

          try {
            await listAgentsApi(testClient, 1);
            console.log('API key verified successfully');
          } catch (error: unknown) {
            console.error(formatApiKeyValidationError(error));
            process.exit(1);
          }

          await setApiKey(apiKey.trim());
          console.log('Login successful! API key saved securely.');
        }
      } catch (error) {
        console.error(`Error during login: ${error}`);
        process.exit(1);
      }
    });
}
