import { Command } from 'commander';
import { render } from 'ink';
import React from 'react';
import LoginView from '../ui/LoginView.js';
import { setApiKey, loadConfig } from '../../shared/config.js';
import { getApiBaseUrl } from '../../shared/elevenlabs-api.js';
import { listAgentsApi } from '../../shared/elevenlabs-api.js';
import { ElevenLabsClient } from '@elevenlabs/elevenlabs-js';

export function createLoginCommand(): Command {
  return new Command('login')
    .description('Login with your ElevenLabs API key')
    .option('--env <environment>', 'Environment name', 'prod')
    .option('--no-ui', 'Disable interactive UI')
    .action(async (options: { ui: boolean; env: string }) => {
      try {
        const environment = options.env || 'prod';

        if (options.ui !== false) {
          // Use Ink UI for login
          const { waitUntilExit } = render(
            React.createElement(LoginView, { environment })
          );
          await waitUntilExit();
        } else {
          // Fallback to text-based login
          const { read } = await import('read');

          console.log(`Logging in to environment: ${environment}`);
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
            const err = error as { statusCode?: number; message?: string; code?: string };
            if (err?.statusCode === 401 || err?.message?.includes('401')) {
              console.error('Invalid API key');
            } else if (err?.code === 'ENOTFOUND' || err?.code === 'ETIMEDOUT' || err?.message?.includes('network')) {
              console.error('Network error: Unable to connect to ElevenLabs API');
            } else {
              console.error('Error verifying API key:', err?.message || error);
            }
            process.exit(1);
          }

          await setApiKey(apiKey.trim(), environment);
          console.log(`Login successful! API key saved securely for environment '${environment}'.`);
        }
      } catch (error) {
        console.error(`Error during login: ${error}`);
        process.exit(1);
      }
    });
}
