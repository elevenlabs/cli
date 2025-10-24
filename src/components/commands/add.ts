import { Command } from 'commander';
import { spawnSync } from 'child_process';
import { URL } from 'url';

export function createAddCommand(): Command {
  return new Command('add')
    .description('Add a component from the ElevenLabs UI registry')
    .argument('[name]', 'Name of the component to add (optional)')
    .action(async (componentName?: string) => {
      try {
        // Check if npx is available
        const npxCheck = spawnSync('npx', ['--version'], { encoding: 'utf-8' });
        if (npxCheck.error) {
          console.error('Error: npx is not available. Please install Node.js/npm.');
          process.exit(1);
        }

        const component = componentName || 'all';
        const targetUrl = new URL(`/r/${component}.json`, 'https://ui.elevenlabs.io').toString();

        console.log(`Installing ${component} from ElevenLabs UI registry...`);
        console.log(`Source: ${targetUrl}\n`);

        // Prepare command with custom registry URL
        const fullCommand = `npx -y shadcn@latest add ${targetUrl}`;

        // Run shadcn add command interactively
        const result = spawnSync(fullCommand, {
          stdio: 'inherit', // Allow interactive prompts
          shell: true
        });

        if (result.error) {
          throw result.error;
        }

        if (result.status !== 0) {
          process.exit(result.status || 1);
        }
      } catch (error) {
        console.error(`Error adding component: ${error}`);
        process.exit(1);
      }
    });
}
