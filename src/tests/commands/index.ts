import { Command } from 'commander';
import { createAddCommand } from './add.js';
import { createDeleteCommand } from './delete.js';
import { createPushCommand } from './push.js';
import { createPullCommand } from './pull.js';

export function createTestsCommand(): Command {
  const tests = new Command('tests');
  tests.description('Manage ElevenLabs tests');

  tests.addCommand(createAddCommand());
  tests.addCommand(createDeleteCommand());
  tests.addCommand(createPushCommand());
  tests.addCommand(createPullCommand());

  return tests;
}
