import { Command } from 'commander';
import { spawnSync } from 'child_process';

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

        console.log('Launching shadcn/ui CLI...');
        console.log('Source: https://ui.elevenlabs.io\n');

        // Prepare command arguments
        const args = ['shadcn@latest', 'add'];
        if (componentName) {
          args.push(componentName);
        }

        // Run shadcn add command interactively
        const result = spawnSync('npx', args, {
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
