import { Command } from 'commander';
import { bashCompletion, zshCompletion } from '../scripts.js';

export function createCompletionCommand(): Command {
  const cmd = new Command('completion')
    .description('Generate shell completion script')
    .argument('<shell>', 'Shell type: bash or zsh')
    .action((shell: string) => {
      const shellLower = shell.toLowerCase();
      
      if (shellLower === 'bash') {
        console.log(bashCompletion);
      } else if (shellLower === 'zsh') {
        console.log(zshCompletion);
      } else {
        console.error(`Unsupported shell: ${shell}`);
        console.error('Supported shells: bash, zsh');
        process.exit(1);
      }
    });

  return cmd;
}
