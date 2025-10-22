import { Command } from 'commander';
import { createAddCommand } from './add.js';

export function createComponentsCommand(): Command {
  const components = new Command('components');
  components.description('Import components from the ElevenLabs UI registry (https://ui.elevenlabs.io)');

  components.addCommand(createAddCommand());

  return components;
}
